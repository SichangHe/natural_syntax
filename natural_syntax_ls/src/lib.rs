use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use drop_this::*;
use natural_syntax::{POSModel, POSToken};
use ropey::Rope;
use tokio::{
    io::{stdin, stdout},
    sync::oneshot,
    task::{block_in_place, spawn_blocking},
};
use tokio_gen_server::prelude::*;
use tower_lsp::{
    jsonrpc::Result as JsonRes, lsp_types::*, Client, LanguageServer, LspService, Server,
};

mod document_registry;
mod semantic_tokens;

use document_registry::*;
use semantic_tokens::*;
use tracing::{debug, error, info};

/// Run the Part of Speech Language Server that provides highlighting.
pub async fn run_part_of_speech_ls() -> Result<()> {
    let model = block_in_place(POSModel::try_default)?;
    info!("Model loaded.");
    let (service, socket) = LspService::build(|client| POSLS::new(client, model)).finish();
    Server::new(stdin(), stdout(), socket).serve(service).await;
    Ok(())
}

pub struct POSLS {
    client: Client,
    document_registry: ActorRef<DocumentRegistry>,
}

const TOKEN_SCORE_THRESHOLD: f64 = 1. / 3.;

impl POSLS {
    pub fn new(client: Client, model: POSModel) -> Self {
        let document_registry = DocumentRegistry::new(Arc::new(model));
        Self {
            client,
            document_registry: document_registry.spawn().1,
        }
    }

    async fn on_change(&self, item: TextItem) {
        self.document_registry
            .cast(DocumentInfo::Item(item))
            .await
            .unwrap();
    }
}

fn predict(model: Arc<POSModel>, item: TextItem, actor_ref: ActorRef<DocumentRegistry>) {
    debug!(uri = item.uri.path(), item.version, "Predicting.");
    let mut tokens = model
        .predict(&item.text)
        .filter_map(|maybe_token| match maybe_token {
            Ok(token) => Some(token),
            Err(err) => {
                error!(?err, ?item.uri, "Tagging text.");
                None
            }
        })
        .filter(filter_token)
        .collect::<Vec<_>>();
    tokens.sort_by_key(|token| token.offset_begin);
    let document = Document {
        text: Rope::from_str(&item.text),
        tokens,
        version: item.version,
    };
    actor_ref
        .blocking_cast(DocumentInfo::Predicted(item.uri, document))
        .drop_result();
}

/// Filter out tokens with low score or purely punctuations.
pub fn filter_token(token: &POSToken) -> bool {
    token.score > TOKEN_SCORE_THRESHOLD
        && token.word.chars().any(|char| !char.is_ascii_punctuation())
}

#[tower_lsp::async_trait]
impl LanguageServer for POSLS {
    async fn initialize(&self, _params: InitializeParams) -> JsonRes<InitializeResult> {
        Ok(InitializeResult {
            capabilities: server_capabilities(),
            ..Default::default()
        })
    }

    async fn did_open(
        &self,
        DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri, version, text, ..
            },
        }: DidOpenTextDocumentParams,
    ) {
        info!(uri = uri.path(), "Opened.");
        self.on_change(TextItem { uri, text, version }).await;
    }

    async fn did_change(
        &self,
        DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, version },
            mut content_changes,
        }: DidChangeTextDocumentParams,
    ) {
        info!(uri = uri.path(), "Changed.");
        debug_assert_eq!(content_changes.len(), 1, "We only take full changes.");
        // TODO: Handle partial changes in `range`.
        let TextDocumentContentChangeEvent { text, .. } = content_changes.pop().unwrap();
        self.on_change(TextItem { uri, text, version }).await
    }

    async fn did_close(
        &self,
        DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        }: DidCloseTextDocumentParams,
    ) {
        info!(uri = uri.path(), "Closed.");
        self.document_registry
            .cast(DocumentInfo::Discard(uri))
            .await
            .unwrap();
    }

    async fn semantic_tokens_full(
        &self,
        SemanticTokensParams {
            text_document: TextDocumentIdentifier { uri },
            ..
        }: SemanticTokensParams,
    ) -> JsonRes<Option<SemanticTokensResult>> {
        info!(uri = uri.path(), "Full semantic tokens requested.");
        let maybe_data = self.document_registry.call(uri).await;
        Ok(maybe_data.ok().map(|data| {
            info!("Sending full semantic tokens.");
            SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data,
            })
        }))
    }

    async fn shutdown(&self) -> JsonRes<()> {
        self.client // Placeholder to make `self.client` used.
            .log_message(MessageType::WARNING, "Exiting.")
            .await;
        Ok(())
    }
}

fn semantic_tokens(text: &Rope, tokens: &[POSToken]) -> Vec<SemanticToken> {
    let mut i_prev_start = 0;
    let mut slice = text.slice(..);
    tokens
        .iter()
        .map(|token| {
            let relative_i_char = token.offset_begin - i_prev_start;
            let relative_i_line = slice.char_to_line(relative_i_char as usize);
            let delta_start = match relative_i_line {
                0 => relative_i_char,
                _ => {
                    let i_1st_char_of_line = slice.line_to_char(relative_i_line);
                    relative_i_char - i_1st_char_of_line as u32
                }
            };
            i_prev_start = token.offset_begin;
            slice = slice.slice((relative_i_char as usize)..);
            let type_n_modifiers = token_type_n_modifiers(token.tag);
            SemanticToken {
                delta_line: relative_i_line as u32,
                delta_start,
                length: token.offset_end - token.offset_begin,
                token_type: type_n_modifiers.token_type,
                token_modifiers_bitset: type_n_modifiers.token_modifiers_bitset,
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
struct TextItem {
    uri: Url,
    text: String,
    version: i32,
}

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        // TODO: Implement incremental change.
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: semantic_token_types(),
                    token_modifiers: semantic_token_modifiers(),
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                // NOTE: Neovim does not support `range`, so we do not either.
                ..Default::default()
            },
        )),
        ..Default::default()
    }
}

/// Document.
#[derive(Debug)]
pub struct Document {
    text: Rope,
    tokens: Vec<POSToken>,
    version: i32,
}

#[cfg(test)]
mod tests;

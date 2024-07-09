use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use natural_syntax::{POSModel, POSToken};
use ropey::Rope;
use tokio::{
    io::{stdin, stdout},
    task::{block_in_place, yield_now},
};
use tokio_two_join_set::TwoJoinSet;
use tower_lsp::{
    jsonrpc::Result as JsonRes, lsp_types::*, Client, LanguageServer, LspService, Server,
};

mod semantic_tokens;

use semantic_tokens::*;
use tracing::{error, info};

/// Run the Part of Speech Language Server that provides highlighting.
pub async fn run_part_of_speech_ls() -> Result<()> {
    let model = block_in_place(POSModel::try_default)?;
    info!("Model loaded.");
    let (service, socket) = LspService::build(|client| POSLS::new(client, model)).finish();
    Server::new(stdin(), stdout(), socket).serve(service).await;
    Ok(())
}

pub type Documents = Arc<DashMap<Url, Document>>;

pub struct POSLS {
    client: Client,
    model: Arc<POSModel>,
    documents: Documents,
    prediction_join_sets: DashMap<Url, TwoJoinSet<()>>,
}

const TOKEN_SCORE_THRESHOLD: f64 = 1. / 3.;

impl POSLS {
    pub fn new(client: Client, model: POSModel) -> Self {
        Self {
            client,
            model: Arc::new(model),
            documents: Default::default(),
            prediction_join_sets: Default::default(),
        }
    }

    async fn on_change(&self, item: TextDocumentItem) {
        let uri = item.uri.clone();
        let task = process_document(item, self.model.clone(), self.documents.clone());
        _ = self
            .prediction_join_sets
            .entry(uri)
            .or_default()
            .spawn(task);
    }
}

async fn process_document(
    TextDocumentItem { uri, text, version }: TextDocumentItem,
    model: Arc<POSModel>,
    documents: Documents,
) {
    if block_in_place(|| version_outdated(&documents, &uri, version)) {
        return info!(?uri, version, "Ignoring outdated version.");
    }
    yield_now().await;
    let predictions = block_in_place(|| model.predict(&text));
    yield_now().await;
    let tokens = predictions
        .filter_map(|maybe_token| match maybe_token {
            Ok(token) => Some(token),
            Err(err) => {
                error!(?err, ?uri, "Tagging text.");
                None
            }
        })
        .filter(filter_token)
        .collect();
    if block_in_place(|| version_outdated(&documents, &uri, version)) {
        return info!(
            ?uri,
            version, "Ignoring outdated version after classifying."
        );
    }
    yield_now().await;
    let document = Document {
        text: Rope::from_str(&text),
        tokens,
        version,
    };
    yield_now().await;
    block_in_place(|| documents.insert(uri, document));
}

fn version_outdated(documents: &DashMap<Url, Document>, uri: &Url, version: i32) -> bool {
    matches!(documents.get(uri), Some(doc) if version <= doc.version)
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        // TODO: Partial changes.
        debug_assert_eq!(
            params.content_changes.len(),
            1,
            "We only take full changes."
        );
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.content_changes.pop().unwrap().text,
            version: params.text_document.version,
        })
        .await
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> JsonRes<Option<SemanticTokensResult>> {
        let Some(document) = block_in_place(|| self.documents.get(&params.text_document.uri))
        else {
            info!(?params.text_document.uri, "Document not found for semantic tokens.");
            return Ok(None);
        };
        info!(?params.text_document.uri, "Will send semantic tokens.");
        let (text, tokens) = (&document.text, &document.tokens);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: semantic_tokens(text, tokens),
        })))
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
struct TextDocumentItem {
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
                // range: Some(true), // TODO: Implement `semantic_tokens_range`
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

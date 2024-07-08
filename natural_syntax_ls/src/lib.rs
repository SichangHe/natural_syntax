use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};

mod semantic_tokens;

use semantic_tokens::*;

#[derive(Debug)]
pub struct POSLS {
    client: Client,
}

impl POSLS {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for POSLS {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: server_capabilities(),
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        self.client // Placeholder to make `self.client` used.
            .log_message(MessageType::WARNING, "Exiting.")
            .await;
        Ok(())
    }
}

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        // TODO: Implement incremental change.
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: SemanticTokensLegend {
                    token_types: semantic_token_types(),
                    token_modifiers: semantic_token_modifiers(),
                },
                ..Default::default()
            },
        )),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests;

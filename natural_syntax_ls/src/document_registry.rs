use super::*;

pub struct DocumentRegistry {
    model: Arc<POSModel>,
    documents: HashMap<Url, DocumentStore>,
}

impl DocumentRegistry {
    pub fn new(model: Arc<POSModel>) -> Self {
        Self {
            model,
            documents: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
struct DocumentStore {
    /// The document waiting to be processed.
    queued: Option<TextDocumentItem>,
    /// If a document is being processed.
    processing: bool,
    /// The processed document.
    document: Option<Document>,
    /// Replies to be made after processing the document.
    delayed_replies: Vec<oneshot::Sender<Vec<SemanticToken>>>,
    latest_version: i32,
}

impl Actor for DocumentRegistry {
    type Call = Url;
    type Cast = DocumentInfo;
    type Reply = Vec<SemanticToken>;

    async fn handle_cast(&mut self, msg: Self::Cast, env: &mut ActorEnv<Self>) -> Result<()> {
        match msg {
            DocumentInfo::Item(item) => {
                let store = self.documents.entry(item.uri.clone()).or_default();
                if store.latest_version < item.version {
                    store.latest_version = item.version;
                    schedule_document_processing(item, store, &self.model, &env.ref_);
                }
            }
            DocumentInfo::Predicted(uri, document) => {
                let store = self.documents.entry(uri.clone()).or_default();
                store.processing = false;
                if !store.delayed_replies.is_empty() {
                    let tokens = semantic_tokens(&document.text, &document.tokens);
                    for reply in store.delayed_replies.drain(1..) {
                        reply.send(tokens.clone()).drop_result();
                    }
                    store
                        .delayed_replies
                        .pop()
                        .unwrap()
                        .send(tokens)
                        .drop_result();
                }
                match &store.document {
                    None => store.document = Some(document),
                    Some(existing_document) if existing_document.version < document.version => {
                        store.document = Some(document);
                    }
                    _ => info!(?uri, ?document.version, "Ignoring outdated document."),
                }
                if let Some(queued) = store.queued.take() {
                    schedule_document_processing(queued, store, &self.model, &env.ref_);
                }
            }
        }
        Ok(())
    }

    async fn handle_call(
        &mut self,
        msg: Self::Call,
        _env: &mut ActorEnv<Self>,
        reply_sender: oneshot::Sender<Self::Reply>,
    ) -> Result<()> {
        let store = self.documents.entry(msg).or_default();
        match (store.processing, &store.document) {
            (false, Some(Document { text, tokens, .. })) => {
                let tokens = semantic_tokens(text, tokens);
                reply_sender.send(tokens).drop_result();
            }
            _ => store.delayed_replies.push(reply_sender),
        }
        Ok(())
    }
}

fn schedule_document_processing(
    item: TextDocumentItem,
    store: &mut DocumentStore,
    model: &Arc<POSModel>,
    ref_: &ActorRef<DocumentRegistry>,
) {
    if store.processing {
        store.queued = Some(item);
    } else {
        (store.processing, store.queued) = (true, None);
        let (model, actor_ref) = (model.clone(), ref_.clone());
        spawn_blocking(move || predict(model, item, actor_ref));
    }
}

pub enum DocumentInfo {
    /// Document item from the language client.
    Item(TextDocumentItem),
    /// Predicted tokens for the document.
    Predicted(Url, Document),
}

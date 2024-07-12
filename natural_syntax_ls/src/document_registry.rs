use super::*;

mod two;

use two::*;

pub struct DocumentRegistry {
    model: Arc<POSModel>,
    documents: HashMap<Url, DocumentStore>,
    token_map: TokenMap,
}

impl DocumentRegistry {
    pub fn new(model: Arc<POSModel>) -> Self {
        Self {
            model,
            documents: Default::default(),
            token_map: Default::default(),
        }
    }
}

#[derive(Debug)]
struct DocumentStore {
    /// The document waiting to be processed.
    queued: Option<TextItem>,
    /// If a document is being processed.
    processing: bool,
    /// The processed document.
    document: Option<Document>,
    /// Replies to be made after processing the document.
    delayed_replies: Two<oneshot::Sender<Vec<SemanticToken>>>,
    latest_version: i32,
}

impl Default for DocumentStore {
    fn default() -> Self {
        Self {
            queued: Default::default(),
            processing: Default::default(),
            document: Default::default(),
            delayed_replies: Default::default(),
            latest_version: i32::MIN,
        }
    }
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
                    debug!(
                        uri = item.uri.path(),
                        item.version, "Scheduling processing latest-version document."
                    );
                    store.latest_version = item.version;
                    schedule_document_processing(item, store, &self.model, &env.ref_);
                }
            }
            DocumentInfo::Predicted(uri, document) => {
                debug!(uri = uri.path(), document.version, "Received prediction.");
                if let Some(store) = self.documents.get_mut(&uri) {
                    store.processing = false;
                    let maybe_reply = match store.queued {
                        // Leave one reply for after the new document is processed.
                        Some(_) => store.delayed_replies.take_older(),
                        None => store.delayed_replies.take_newer_n_clear(),
                    };
                    if let Some(reply) = maybe_reply {
                        debug!(uri = uri.path(), "Sending delayed reply.");
                        let tokens =
                            semantic_tokens(&document.text, &document.tokens, &self.token_map);
                        reply.send(tokens).drop_result();
                    }
                    store.document = Some(document);
                    if let Some(queued) = store.queued.take() {
                        schedule_document_processing(queued, store, &self.model, &env.ref_);
                    }
                } else {
                    debug!("Discarding uninteresting prediction.");
                }
            }
            DocumentInfo::Discard(uri) => _ = self.documents.remove(&uri),
            DocumentInfo::TokenMapUpdate(update) => self.token_map.extend(update),
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
                let tokens = semantic_tokens(text, tokens, &self.token_map);
                reply_sender.send(tokens).drop_result();
            }
            _ => _ = store.delayed_replies.push(reply_sender),
        }
        Ok(())
    }
}

fn schedule_document_processing(
    item: TextItem,
    store: &mut DocumentStore,
    model: &Arc<POSModel>,
    ref_: &ActorRef<DocumentRegistry>,
) {
    if store.processing {
        debug!(
            uri = item.uri.path(),
            item.version, "Queuing for prediction."
        );
        store.queued = Some(item);
    } else {
        (store.processing, store.queued) = (true, None);
        let (model, actor_ref) = (model.clone(), ref_.clone());
        spawn_blocking(move || predict(model, item, actor_ref));
    }
}

pub enum DocumentInfo {
    /// Document item from the language client.
    Item(TextItem),
    /// Predicted tokens for the document.
    Predicted(Url, Document),
    /// Forget about the document.
    Discard(Url),
    /// Instruction to update the token map.
    TokenMapUpdate(HashMap<PartOfSpeech, Option<TokenTypeNModifiers>>),
}

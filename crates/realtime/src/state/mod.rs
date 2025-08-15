use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crdt::Rga;

use crate::state::document::Document;

pub mod document;

#[derive(Default)]
pub struct AppState {
    documents: Mutex<HashMap<String, Arc<Document>>>,
}

impl AppState {
    pub fn insert_document(&self, id: String, state: Rga<char>) -> Arc<Document> {
        let document = Arc::new(Document::new(state));
        self.documents
            .lock()
            .unwrap()
            .insert(id, Arc::clone(&document));
        document
    }

    pub fn get_document(&self, id: &str) -> Option<Arc<Document>> {
        self.documents.lock().unwrap().get(id).cloned()
    }
}

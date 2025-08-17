use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};

use crdt::{Rga, VersionVector};
use tokio::sync::broadcast;

use crate::events::RealtimeEvent;

const DOCUMENT_BROADCAST_CAPACITY: usize = 4096;

pub struct Document {
    state: Mutex<Rga<char>>,
    max_actor_id: AtomicU64,
    sender: broadcast::Sender<RealtimeEvent>,
    receiver: broadcast::Receiver<RealtimeEvent>,
}

impl Document {
    pub fn new(state: Rga<char>) -> Self {
        let (sender, receiver) = broadcast::channel(DOCUMENT_BROADCAST_CAPACITY);
        Self {
            state: Mutex::new(state),
            sender,
            max_actor_id: Default::default(),
            receiver,
        }
    }

    pub fn change<R>(&self, action: impl FnOnce(&mut Rga<char>) -> R) -> R {
        action(&mut self.state.lock().unwrap())
    }

    pub fn version(&self) -> VersionVector {
        self.state.lock().unwrap().version()
    }

    pub fn run_compaction(&self) {
        self.change(|state| state.compact());
    }

    pub fn sender(&self) -> &broadcast::Sender<RealtimeEvent> {
        &self.sender
    }

    pub fn receiver(&self) -> &broadcast::Receiver<RealtimeEvent> {
        &self.receiver
    }

    pub fn new_actor(&self) -> u64 {
        self.max_actor_id.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn remove_actor(&self, id: u64) {
        if self.max_actor_id.load(Ordering::Relaxed) <= id {
            self.max_actor_id.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

use std::{
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use crdt::Rga;
use tokio::sync::broadcast;

use crate::events::RealtimeEvent;

const DOCUMENT_BROADCAST_CAPACITY: usize = 4096;
// 60 seconds
const DOCUMENT_COMPACTION_MIN_INTERVAL: u64 = 60000;

pub struct Document {
    state: Mutex<Rga<char>>,
    num_actors: AtomicU64,
    last_compaction: AtomicU64,
    sender: broadcast::Sender<RealtimeEvent>,
    receiver: broadcast::Receiver<RealtimeEvent>,
}

impl Document {
    pub fn new(state: Rga<char>) -> Self {
        let (sender, receiver) = broadcast::channel(DOCUMENT_BROADCAST_CAPACITY);
        Self {
            state: Mutex::new(state),
            sender,
            num_actors: Default::default(),
            receiver,
            last_compaction: Default::default(),
        }
    }

    pub fn change(&self, action: impl FnOnce(&mut Rga<char>)) {
        action(&mut self.state.lock().unwrap())
    }

    pub fn run_compaction(&self) {
        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
            - self.last_compaction.load(Ordering::Relaxed)
            < DOCUMENT_COMPACTION_MIN_INTERVAL
        {
            return;
        }
        self.last_compaction.store(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            Ordering::Relaxed,
        );
        self.change(|state| state.compact());
    }

    pub fn sender(&self) -> &broadcast::Sender<RealtimeEvent> {
        &self.sender
    }

    pub fn receiver(&self) -> &broadcast::Receiver<RealtimeEvent> {
        &self.receiver
    }

    pub fn new_actor(&self) -> u64 {
        self.num_actors.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn remove_actor(&self) {
        self.num_actors.fetch_sub(1, Ordering::Relaxed);
    }
}

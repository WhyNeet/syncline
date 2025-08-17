use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use crdt::{Rga, VersionVector};
use tokio::sync::broadcast;

use crate::events::RealtimeEvent;

const DOCUMENT_BROADCAST_CAPACITY: usize = 4096;

pub struct Document {
    state: Arc<Mutex<Rga<char>>>,
    max_actor_id: AtomicU64,
    sender: broadcast::Sender<RealtimeEvent>,
    receiver: broadcast::Receiver<RealtimeEvent>,
    compaction_sender: broadcast::Sender<()>,
}

impl Document {
    pub fn new(state: Rga<char>) -> Self {
        let (sender, receiver) = broadcast::channel(DOCUMENT_BROADCAST_CAPACITY);
        let (compaction_sender, _) = broadcast::channel(DOCUMENT_BROADCAST_CAPACITY);
        let state = Arc::new(Mutex::new(state));

        let cs_clone = compaction_sender.clone();
        let task_state = Arc::clone(&state);
        tokio::spawn(async move {
            let sender = cs_clone;
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                task_state.lock().unwrap().compact();
                sender.send(()).unwrap();
            }
        });

        Self {
            state,
            sender,
            max_actor_id: Default::default(),
            receiver,
            compaction_sender,
        }
    }

    pub fn change<R>(&self, action: impl FnOnce(&mut Rga<char>) -> R) -> R {
        action(&mut self.state.lock().unwrap())
    }

    pub fn version(&self) -> VersionVector {
        self.state.lock().unwrap().version()
    }

    pub fn on_compaction(&self) -> broadcast::Receiver<()> {
        self.compaction_sender.subscribe()
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

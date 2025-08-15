use std::sync::Mutex;

use crdt::Rga;
use tokio::sync::broadcast;

use crate::events::RealtimeEvent;

const DOCUMENT_BROADCAST_CAPACITY: usize = 4096;

pub struct Document {
    state: Mutex<Rga<char>>,
    num_actors: Mutex<u64>,
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
        }
    }

    pub fn change(&self, action: impl FnOnce(&mut Rga<char>)) {
        action(&mut self.state.lock().unwrap())
    }

    pub fn sender(&self) -> &broadcast::Sender<RealtimeEvent> {
        &self.sender
    }

    pub fn receiver(&self) -> &broadcast::Receiver<RealtimeEvent> {
        &self.receiver
    }

    pub fn new_actor(&self) -> u64 {
        let mut actors = self.num_actors.lock().unwrap();
        *actors += 1;
        *actors
    }

    pub fn remove_actor(&self) {
        let mut actors = self.num_actors.lock().unwrap();
        if let Some(num_actors) = actors.checked_sub(1) {
            *actors = num_actors;
        }
    }
}

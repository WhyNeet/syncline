use crdt::{ActorId, RgaInsertQuery, RgaUnitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum RealtimeEventKind {
    Insert {
        id: RgaUnitId,
        contents: char,
        query: RgaInsertQuery,
    },
    Delete {
        id: RgaUnitId,
    },
}

#[derive(Debug, Clone)]
pub struct RealtimeEvent {
    pub actor: ActorId,
    pub kind: RealtimeEventKind,
}

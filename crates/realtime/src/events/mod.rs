use crdt::{ActorId, RgaInsertQuery, RgaUnitId, VersionVector};
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
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeEvent {
    pub actor: ActorId,
    pub version: VersionVector,
    pub kind: RealtimeEventKind,
}

use crdt::{RgaInsertQuery, RgaUnitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum RealtimeEvent {
    Insert {
        id: RgaUnitId,
        contents: char,
        query: RgaInsertQuery,
    },
}

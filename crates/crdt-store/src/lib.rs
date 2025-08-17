use std::fmt::Debug;

use crdt::{ActorId, Rga, RgaUnit, RgaUnitId};
use serde::{Deserialize, Serialize};

pub struct RgaSerializer;

impl RgaSerializer {
    pub fn to_vec<T: Debug + Default + Clone>(rga: Rga<T>) -> Vec<RgaStoreUnit<T>> {
        let mut result = vec![];

        let mut unit = rga.root();

        while let Some(next) = unit.next.as_ref() {
            result.push(RgaStoreUnit {
                id: next.id,
                contents: next.contents.clone(),
                is_tombstone: next.is_tombstone,
            });
            unit = next;
        }

        result
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RgaStoreUnit<T: Debug> {
    pub id: RgaUnitId,
    pub contents: T,
    pub is_tombstone: bool,
}

pub struct RgaDeserializer;

impl RgaDeserializer {
    pub fn from_vec<T: Debug + Default>(vec: Vec<RgaStoreUnit<T>>, actor_id: ActorId) -> Rga<T> {
        let mut rga = Rga::new(actor_id);

        for unit in vec {
            let unit = RgaUnit {
                id: unit.id,
                contents: unit.contents,
                is_tombstone: unit.is_tombstone,
                next: None,
            };

            rga.insert_last(unit);
        }

        rga
    }
}

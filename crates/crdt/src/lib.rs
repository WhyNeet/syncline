use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Rga<T: Debug> {
    root: Box<RgaUnit<T>>,
    clock: ActorClock,
    actor_id: ActorId,
}

impl<T: Default + Debug> Rga<T> {
    pub fn new(actor_id: ActorId) -> Self {
        Self {
            root: Box::new(RgaUnit {
                next: None,
                is_tombstone: false,
                contents: T::default(),
                id: (actor_id, 0),
            }),
            clock: ActorClock::default(),
            actor_id,
        }
    }

    pub fn root(&self) -> &RgaUnit<T> {
        &self.root
    }

    pub fn clock(&self) -> ActorClock {
        self.clock
    }

    pub fn insert(
        &mut self,
        query: RgaInsertQuery,
        contents: T,
        actor_id: Option<ActorId>,
        id: Option<ActorClock>,
    ) -> Option<RgaUnitId> {
        let Some(prev_unit) = (match query {
            RgaInsertQuery::Right(id) => {
                let mut unit = &mut self.root;

                loop {
                    if unit.id == id {
                        let mut prev = unit;
                        break loop {
                            let next = prev.next.as_ref();

                            if next.is_none()
                                || actor_id.unwrap_or(self.actor_id) <= next.unwrap().id.0
                            {
                                break Some(prev);
                            }

                            prev = prev.next.as_mut().unwrap();
                        };
                    }

                    if let Some(ref mut next) = unit.next {
                        unit = next;
                    } else {
                        break None;
                    }
                }
            }
            RgaInsertQuery::Middle(left_id, right_id) => {
                let mut unit = &mut self.root;

                loop {
                    if unit.id == left_id {
                        let next = match unit.next.as_ref() {
                            Some(next) => next,
                            _ => break None,
                        };

                        if next.id != right_id {
                            // resolve conflict
                            let mut prev = unit;

                            break loop {
                                let next = match prev.next.as_ref() {
                                    Some(next) => next,
                                    _ => break None,
                                };

                                if next.id == right_id
                                    || actor_id.unwrap_or(self.actor_id) <= next.id.0
                                {
                                    break Some(prev);
                                }

                                prev = prev.next.as_mut().unwrap();
                            };
                        }

                        break Some(unit);
                    }

                    if let Some(ref mut next) = unit.next {
                        unit = next;
                    } else {
                        break None;
                    }
                }
            }
        }) else {
            return None;
        };

        if let Some(id) = id {
            id
        } else {
            self.clock += 1;
            self.clock
        };
        let tmp_next = prev_unit.next.take();
        let unit_id = (actor_id.unwrap_or(self.actor_id), id.unwrap_or(self.clock));
        let new_unit = RgaUnit {
            contents,
            id: unit_id,
            next: tmp_next,
            is_tombstone: false,
        };
        prev_unit.next.replace(Box::new(new_unit));

        Some(unit_id)
    }

    pub fn delete(&mut self, id: RgaUnitId) {
        if id.1 == 0 {
            // don't delete the root, its empty anyways
            return;
        }

        let mut unit = &mut self.root;

        while unit.id != id {
            if let Some(ref mut next) = unit.next {
                unit = next;
            } else {
                return;
            }
        }

        unit.is_tombstone = true;
    }

    pub fn compact(&mut self) {
        let mut unit = &mut self.root;

        loop {
            let next = match unit.next.as_mut() {
                Some(next) => next,
                _ => break,
            };

            if next.is_tombstone {
                let next_next = next.next.take();
                unit.next = next_next;
            } else if let Some(ref mut next) = unit.next {
                unit = next;
            }
        }
    }
}

impl<T: Default + Debug + Display> fmt::Display for Rga<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(ref unit) = self.root().next else {
            return Ok(());
        };
        let mut unit = unit;
        loop {
            if !unit.is_tombstone {
                write!(f, "{}", unit.contents)?;
            }

            if let Some(ref next) = unit.next {
                unit = next;
            } else {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RgaInsertQuery {
    Right(RgaUnitId),
    Middle(RgaUnitId, RgaUnitId),
}

pub type ActorId = u64;
pub type ActorClock = u64;
pub type RgaUnitId = (ActorId, ActorClock);

#[derive(Debug, Clone)]
pub struct RgaUnit<T: Debug> {
    pub next: Option<Box<RgaUnit<T>>>,
    pub is_tombstone: bool,
    pub contents: T,
    pub id: RgaUnitId,
}

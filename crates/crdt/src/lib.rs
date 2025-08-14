use std::fmt::{self, Debug, Display};

#[derive(Debug)]
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

    pub fn insert(&mut self, query: RgaInsertQuery, contents: T) -> RgaUnitId {
        let Some(prev_unit) = (match query {
            RgaInsertQuery::Left(id) => {
                let mut unit = &mut self.root;

                loop {
                    if unit.id == id {
                        break Some(unit);
                    }

                    if let Some(ref mut next) = unit.next {
                        unit = next;
                    } else {
                        break None;
                    }
                }
            }
            RgaInsertQuery::Right(id) => {
                let mut prev = &mut self.root;

                loop {
                    let next = match prev.next.as_ref() {
                        Some(next) => next,
                        _ => break None,
                    };

                    if next.id == id {
                        break Some(prev);
                    }

                    prev = prev.next.as_mut().unwrap();
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
                            let mut prev = &mut self.root;

                            break loop {
                                let next = match prev.next.as_ref() {
                                    Some(next) => next,
                                    _ => break None,
                                };

                                if next.id == right_id || self.actor_id <= next.id.0 {
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
            panic!("no unit with this id exists.");
        };

        self.clock += 1;
        let tmp_next = prev_unit.next.take();
        let id = (self.actor_id, self.clock);
        let new_unit = RgaUnit {
            contents,
            id,
            next: tmp_next,
            is_tombstone: false,
        };
        prev_unit.next.replace(Box::new(new_unit));

        id
    }
}

impl<T: Default + Debug + Display> fmt::Display for Rga<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(ref unit) = self.root().next else {
            return Ok(());
        };
        let mut unit = unit;
        loop {
            write!(f, "{}", unit.contents)?;

            if let Some(ref next) = unit.next {
                unit = next;
            } else {
                break;
            }
        }

        Ok(())
    }
}

pub enum RgaInsertQuery {
    Left(RgaUnitId),
    Right(RgaUnitId),
    Middle(RgaUnitId, RgaUnitId),
}

pub type ActorId = u64;
pub type ActorClock = u64;
pub type RgaUnitId = (ActorId, ActorClock);

#[derive(Debug)]
pub struct RgaUnit<T: Debug> {
    pub next: Option<Box<RgaUnit<T>>>,
    pub is_tombstone: bool,
    pub contents: T,
    pub id: RgaUnitId,
}

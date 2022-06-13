use super::SpawnJob;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, SlotMap};

new_key_type! {
    pub struct SpawnTicket;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpawnPool {
    jobs: SlotMap<SpawnTicket, SpawnJob>,
}

impl SpawnPool {
    pub fn new() -> Self {
        SpawnPool {
            jobs: SlotMap::with_key(),
        }
    }

    pub fn add(&mut self, job: SpawnJob) -> SpawnTicket {
        todo!()
    }

    pub fn collect(&mut self, ticket: SpawnTicket) -> Option<Entity> {
        None
    }

    pub fn collect_all(&mut self) -> Vec<(SpawnTicket, Entity)> {
        Vec::new()
    }

    pub fn cancel_ticket(&mut self, ticket: SpawnTicket) -> bool {
        false
    }
}

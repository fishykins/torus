use super::EntityType;
use crate::bundles::PlayerBundle;
use bevy::prelude::*;
use fishics::components::PhysicsMaterial;
use prima::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use torus_common::components::Agent;

#[derive(Clone, Serialize, Deserialize)]
pub struct Spawn {
    pub owner: Option<u64>,
    pub uuid: u64,
    pub r#type: EntityType,
    pub position: Point<f32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpawnPool {
    pub spawns: Vec<Spawn>,
    pub uuid: u64,
}

impl SpawnPool {
    pub fn new() -> Self {
        SpawnPool {
            spawns: Vec::new(),
            uuid: 0,
        }
    }

    pub fn add(
        &mut self,
        entity: EntityType,
        position: Point<f32>,
        owner: Option<u64>,
        uuid: Option<u64>,
    ) {
        let spawn = Spawn {
            owner,
            uuid: uuid.unwrap_or_else(|| self.new_uuid()),
            r#type: entity,
            position,
        };
        self.spawns.push(spawn);
    }

    pub fn new_uuid(&mut self) -> u64 {
        let uuid = self.uuid;
        self.uuid += 1;
        uuid
    }
}

impl Spawn {
    pub fn instantiate(
        self,
        mut commands: Commands,
        id: u64,
        material: Handle<PhysicsMaterial>,
    ) -> Entity {
        let bundle = match self.r#type {
            EntityType::Biped => PlayerBundle::new(
                format!("{}", &self),
                Agent::new(self.owner, id),
                self.position,
                material,
            ),
        };
        commands.spawn_bundle(bundle).id()
    }
}

impl Display for Spawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(owner) = self.owner {
            write!(f, "{}_{}", self.r#type, owner)
        } else {
            write!(f, "{}", self.r#type)
        }
    }
}

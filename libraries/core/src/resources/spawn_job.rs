use super::EntityType;
use crate::bundles::PlayerBundle;
use bevy::prelude::*;
use fishics::components::PhysicsMaterial;
use prima::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use torus_common::components::Agent;

pub type SpawnTicket = u32;

#[derive(Clone, Serialize, Deserialize)]
pub struct SpawnJob {
    pub owner: Option<u64>,
    pub uuid: u64,
    pub r#type: EntityType,
    pub position: Point<f32>,
}

impl SpawnJob {
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

impl Display for SpawnJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(owner) = self.owner {
            write!(f, "{}_{}", self.r#type, owner)
        } else {
            write!(f, "{}", self.r#type)
        }
    }
}

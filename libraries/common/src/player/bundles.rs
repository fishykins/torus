use crate::{player::components::*, Uuid};
use bevy::core::Name;
use bevy::prelude::*;
use fishics::{components::{Collider, RigidBody, PhysicsMaterial}, bundles::RigidBodyBundle};
use prima::prelude::Point;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub uuid: Uuid,
    pub biped: Biped,
    pub controller: Controller,
    #[bundle]
    pub rigid_body: RigidBodyBundle,
}

impl PlayerBundle {
    pub fn new(name: String, position: Point<f32>, material: Handle<PhysicsMaterial>) -> Self {
        let rigid_body = RigidBodyBundle {
            rb: RigidBody::new(position),
            collider: Collider::circle(0.5),
            properties: material,
            ..Default::default()
        };

        Self {
            name: Name::new(name),
            uuid: Uuid::default(),
            biped: Biped::default(),
            controller: Controller::default(),
            rigid_body,
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new("Player".to_string(), Point::new(0.0f32, 0.0f32), Handle::default())
    }
}
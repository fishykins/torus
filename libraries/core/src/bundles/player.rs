use torus_common::{Uuid, components::{Biped, Controller, Agent}};
use bevy::core::Name;
use bevy::prelude::*;
use fishics::{
    bundles::RigidBodyBundle,
    components::{Collider, ColliderRender, PhysicsMaterial, RigidBody},
};
use prima::prelude::Point;
use torus_physics::components::Drag;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub uuid: Uuid,
    pub biped: Biped,
    pub agent: Agent,
    pub controller: Controller,
    pub drag: Drag,
    #[bundle]
    pub rigid_body: RigidBodyBundle,
}

impl PlayerBundle {
    pub fn new(name: String, agent: Agent, position: Point<f32>, material: Handle<PhysicsMaterial>) -> Self {
        let rigid_body = RigidBodyBundle {
            rb: RigidBody::new(position),
            collider: Collider::circle(0.5),
            render: ColliderRender {
                colour: Color::rgb(0.1, 0.5, 0.7),
            },
            properties: material,
            ..Default::default()
        };

        Self {
            name: Name::new(name),
            uuid: Uuid::default(),
            biped: Biped::default(),
            controller: Controller::default(),
            drag: Drag::default(),
            agent,
            rigid_body,
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new(
            "Player".to_string(),
            Agent::default(),
            Point::new(0.0f32, 0.0f32),
            Handle::default(),
        )
    }
}

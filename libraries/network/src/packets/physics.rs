use std::collections::HashMap;


use super::Packet;
use bevy::prelude::*;
use fishics::components::{RigidBody, Velocity};
use prima::prelude::*;
use serde::{Deserialize, Serialize};
use torus_common::components::Agent;

/// Contains positional/velocity data for an entity.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PhysicsPacket {
    position: Vec2,
    rotation: f32,
    linear_velocity: Vector<f32>,
    angular_velocity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsPackets(HashMap<Agent, PhysicsPacket>);

impl PhysicsPacket {
    pub fn new(rb: &RigidBody, velocity: &Velocity) -> Self {
        Self {
            position: rb.position,
            rotation: rb.rotation,
            linear_velocity: velocity.linear(),
            angular_velocity: velocity.angular(),
        }
    }
}

impl PhysicsPackets {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, agent: &Agent, rb: &RigidBody, velocity: &Velocity) {
        self.0.insert(agent.clone(), PhysicsPacket::new(rb, velocity));
    }

    pub fn get(&self, agent: &Agent) -> Option<&PhysicsPacket> {
        self.0.get(agent)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Agent, &PhysicsPacket)> {
        self.0.iter()
    }
}

impl Packet for PhysicsPacket {
    fn decode(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}

impl Packet for PhysicsPackets {
    fn decode(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
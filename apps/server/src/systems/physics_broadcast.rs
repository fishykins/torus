use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use fishics::components::{RigidBody, Velocity};
use torus_common::components::Agent;
use torus_network::{packets::{PhysicsPackets, Packet}, CHANNEL_PHYSICS};


pub fn physics_broadcast_system(
    mut server: ResMut<RenetServer>,
    player_query: Query<(&Agent, &RigidBody, &Velocity)>,
) {
    let mut physics_packets = PhysicsPackets::new();

    for (agent, body, velocity) in player_query.iter() {
        physics_packets.add(agent, body, velocity);
    }
    server.broadcast_message(CHANNEL_PHYSICS, physics_packets.encode());
}

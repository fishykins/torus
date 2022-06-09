use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use prima::prelude::*;
use torus_core::entities::{SpawnPool, EntityType};

use crate::resources::{Lobby, PlayerStatus};

/// Handles active client connections.
pub fn connections_handler(
    server: Res<RenetServer>,
    mut lobby: ResMut<Lobby>,
    mut spawn_pool: ResMut<SpawnPool>,
) {
    for client_id in server.clients_id().into_iter() {
        if let Some(player) = lobby.get_player_mut(client_id) {
            match player.status {
                PlayerStatus::Unasigned => {
                    // Assume we can just spawn.
                    player.status = PlayerStatus::Waiting;
                    spawn_pool.add(EntityType::Biped, Point::new(0.0, 0.0), Some(client_id), None);
                }
                _ => {}
            }
        }
    }
}

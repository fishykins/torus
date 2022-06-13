use bevy::prelude::*;
use torus_core::resources::SpawnPool;

use crate::resources::{Lobby, PlayerStatus};

/// Handles players in the lobby pool.
pub fn lobby_manager(mut lobby: ResMut<Lobby>, mut spawn_pool: ResMut<SpawnPool>) {
    for client_id in lobby.players() {
        if let Some(player) = lobby.get_player_mut(client_id) {
            match player.status {
                PlayerStatus::Unasigned => {
                    // Assume we can just spawn.
                    player.status = PlayerStatus::Waiting;
                }
                _ => {}
            }
        }
    }
}

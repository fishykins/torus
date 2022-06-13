use std::collections::HashMap;

use bevy::prelude::*;

pub struct Player {
    pub status: PlayerStatus,
}

pub struct Lobby {
    players: HashMap<u64, Player>,
}

pub enum PlayerStatus {
    Unasigned,
    Waiting,
    Assigned(Entity),
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, id: u64) -> &mut Self {
        let player = Player {
            status: PlayerStatus::Unasigned,
        };
        self.players.insert(id, player);
        self
    }

    pub fn remove_player(&mut self, id: u64) {
        self.players.remove(&id);
    }

    pub fn get_player(&self, id: u64) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn get_player_mut(&mut self, id: u64) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    pub fn assign(&mut self, id: u64, entity: Entity) -> bool {
        if let Some(player) = self.players.get_mut(&id) {
            player.status = PlayerStatus::Assigned(entity);
            println!("Player {} assigned.", id);
            true
        } else {
            println!("Player {} not found: failed to assign.", id);
            false
        }
    }

    pub fn players(&self) -> Vec<u64> {
        self.players.keys().cloned().collect()
    }
}

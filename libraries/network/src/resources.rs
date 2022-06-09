use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}
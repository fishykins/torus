use std::fmt::Display;
use bevy_inspector_egui::Inspectable;
use serde::{Serialize, Deserialize};

mod spawn;

pub use spawn::*;


#[derive(Debug, Clone, Inspectable, Serialize, Deserialize)]
pub enum EntityType {
    Biped,
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Biped => write!(f, "Biped"),
        }
    }
}
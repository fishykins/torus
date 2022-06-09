use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Serialize, Deserialize};


/// Input values assigned to this entity. This can be from an AI or player input.
#[derive(Debug, Default, Clone, Component, Inspectable, Serialize, Deserialize)]
pub struct Controller {
    pub target: Option<Vec2>,
    pub movement: Vec2,
}
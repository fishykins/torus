use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;


/// Input values assigned to this entity. This can be from an AI or player input.
#[derive(Debug, Default, Clone, Component, Inspectable)]
pub struct Controller {
    pub target: Option<Vec2>,
    pub movement: Vec2,
}
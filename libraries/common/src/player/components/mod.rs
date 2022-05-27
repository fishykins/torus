mod agent;
mod biped;
mod controller;

pub use agent::Agent;
pub use biped::Biped;
pub use controller::Controller;



use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Clone, Component, Inspectable)]
pub struct Player {}
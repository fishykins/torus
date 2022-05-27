use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct Drag {
    pub grounded: bool,
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            grounded: true,
        }
    }
}
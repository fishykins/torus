use crate::player::systems::*;
use super::components::*;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

pub struct PlayerPlugin {}

impl Default for PlayerPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Biped>()
            .register_inspectable::<Controller>();

        app.add_system(biped_movement_system);
    }
}

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use super::{components::*, systems::*};
use fishics::{systems::core::integration, network::LocalNet};

pub struct PhysicsPlugin {

}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {

        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Drag>();
        app.add_system(drag_system.after(integration::<LocalNet>));
    }
}
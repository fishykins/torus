use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use super::items::*;

pub struct ItemPlugin {

}

impl Default for ItemPlugin {
    fn default() -> Self {
        Self {

        }
    }
}

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<ItemContainer>();
    }
}
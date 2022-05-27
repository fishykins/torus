use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const BASE_SPEED: f32 = 20000.0;
const TURN_SPEED: f32 = 4.0;
//const MASS: f32 = 64.0;

/// A simplistic bipedal character. Holds basic movement values.
#[derive(Debug, Clone, Copy, Inspectable, Reflect, Component)]
pub struct Biped {
    pub movement_speed: f32,
    pub turn_speed: f32,
}

impl Default for Biped {
    fn default() -> Self {
        Self {
            movement_speed: BASE_SPEED,
            turn_speed: TURN_SPEED,
        }
    }
}

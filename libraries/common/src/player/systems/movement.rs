use fishics::components::*;
use bevy::prelude::*;
use crate::player::components::*;
use prima::prelude::*;

pub fn biped_movement_system(
    mut bipeds: Query<(Entity, &mut Biped, &mut Forces, &RigidBody, &Controller)>,
) {
    for (entity, biped, mut forces, rb, controller) in bipeds.iter_mut() {
        let current_dir = Rotation::new(rb.rotation);
        let movement = Vector::new(controller.movement.x, controller.movement.y);
        if !movement.is_zero() {
            forces.add_impulse(movement * biped.movement_speed);
        }
    }
}
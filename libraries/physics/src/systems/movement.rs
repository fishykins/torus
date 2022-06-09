use torus_common::components::*;
use bevy::prelude::*;
use fishics::{components::*, resources::FishicsConfig};
use prima::prelude::*;

pub fn biped_movement_system(
    mut bipeds: Query<(
        Entity,
        &mut Biped,
        &mut Forces,
        &mut RigidBody,
        &Controller,
        &Velocity,
    )>,
    cfg: Res<FishicsConfig>,
    time: Res<Time>,
) {
    for (_entity, biped, mut forces, mut rb, controller, velocity) in bipeds.iter_mut() {
        let movement = Vector::new(controller.movement.x, controller.movement.y);

        if !movement.is_zero() {
            let current_dir = Angle::new(rb.rotation);
            let target_direction = movement.as_angle();
            let new_dir = current_dir.lerp(
                &target_direction,
                0.92 * cfg.time * time.delta_seconds() * biped.turn_speed,
            );
            let impulse = new_dir.as_vector() * biped.movement_speed;

            rb.rotation = new_dir.as_radians();
            forces.add_impulse(impulse);
        } else {
            let forward = velocity.linear().inverted();
            forces.add_impulse(forward * 0.1);
        }
    }
}

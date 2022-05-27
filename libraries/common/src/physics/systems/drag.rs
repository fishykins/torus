use crate::physics::components::Drag;
use bevy::prelude::*;
use fishics::components::{PhysicsMaterial, Velocity};

pub fn drag_system(
    materials: Res<Assets<PhysicsMaterial>>,
    mut query: Query<(&Handle<PhysicsMaterial>, &mut Velocity, &Drag)>,
) {
    for (mat, mut vel, drag) in query.iter_mut() {
        if drag.grounded {
            if let Some(mat) = materials.get(mat) {
                if mat.restitution > 0.0 {
                    let friction_force = vel.linear() * (1.0 - mat.restitution);
                    vel.sub_linear(friction_force);
                }
            }
        }
    }
}

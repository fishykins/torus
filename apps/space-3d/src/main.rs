use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, physics::TimestepMode};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_startup_system(enable_physics_profiling.system())
        .run();
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn setup_graphics(mut commands: Commands) {
    const HALF_SIZE: f32 = 100.0;

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 100.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 2.0, 10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::face_toward(
            Vec3::new(-30.0, 30.0, 100.0),
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )),
        ..Default::default()
    });
    commands.insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)));
}

pub fn setup_physics(mut commands: Commands) {
    let physics_config = RapierConfiguration {
        gravity: Vector::zeros(),
        scale: 1.0,
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::VariableTimestep,
    };

    commands.insert_resource(physics_config);

    // Build the rigid body.
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 0.0, 0.0).into(),
        ..RigidBodyBundle::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
        ..ColliderBundle::default()
    };

    commands
        .spawn()
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(ColliderDebugRender::with_id(76))
        .insert(ColliderPositionSync::Discrete);
}

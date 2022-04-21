use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use fishics::{
    impulse_resolution, Collider, FishicsPlugin, Mass, PhysicsMaterial, RigidBody, RigidBodyBundle,
    Velocity,
};
use prima::{Point};

const SCALE: f32 = 20.0;
const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 300.0;
const MAX_SPEED: f32 = 200.0;

fn main() {
    // Debug settings
    let mut log_setting = LogSettings::default();
    log_setting.level = Level::WARN;

    // Build the app
    let mut app = App::new();

    // Add main plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FishicsPlugin::default());

    // Insert resources
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.12)))
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Torus client".to_string(),
            ..Default::default()
        })
        .insert_resource(log_setting);

    app.add_startup_system(setup);

    app.add_system(update_transform.after(impulse_resolution))
        .add_system(reset_cubes);

    // Run the app
    app.run();
}

fn update_transform(mut bodies: Query<(&mut Transform, &RigidBody)>) {
    for (mut transform, rigid_body) in bodies.iter_mut() {
        let pos = Vec3::new(
            rigid_body.position.x * SCALE,
            rigid_body.position.y * SCALE,
            0.0,
        );
        transform.translation = pos;
    }
}

fn reset_cubes(mut cubes: Query<(&mut RigidBody, &mut Velocity)>) {
    for (mut rb, mut vel) in cubes.iter_mut() {
        let x = rb.position().x;
        let y = rb.position().y;

        if x > SCREEN_WIDTH / SCALE * 1.2 {
            rb.set_position(Point::new(-SCREEN_WIDTH / SCALE, y));
        }
        if x < -SCREEN_WIDTH / SCALE * 1.2 {
            rb.set_position(Point::new(SCREEN_WIDTH / SCALE, y));
        }
        if y > SCREEN_HEIGHT / SCALE * 1.2 {
            rb.set_position(Point::new(x, -SCREEN_HEIGHT / SCALE));
        }
        if y < -SCREEN_HEIGHT / SCALE * 1.2 {
            rb.set_position(Point::new(x, SCREEN_HEIGHT / SCALE));
        }

        let v = vel.linear();
        // if v.magnitude() > 0.0001 {
        //     vel.set_linear(v * 0.99);
        // } else {
        //     vel.set_linear(Vector2::zero());
        // }

        if v.magnitude() > MAX_SPEED {
            vel.set_linear(v.normalize() * MAX_SPEED);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    spawn_cube(
        &mut commands,
        Point::new(5.0, 0.0),
        Color::rgb(0.5, 0.5, 0.4),
        4.0,
    );
    spawn_cube(
        &mut commands,
        Point::new(-5.0, 0.0),
        Color::rgb(0.24, 0.5, 0.1),
        2.0,
    );
}

fn spawn_cube(commands: &mut Commands, pos: Point<f32>, colour: Color, size: f32) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: colour,
                custom_size: Some(Vec2::new(1.0, 1.0) * size * SCALE),
                ..default()
            },
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: Collider::square(size),
            material: PhysicsMaterial::soft(),
            velocity: Velocity::new(
                Vec2::new(10.0 - 20.0 * colour.r(), 10.0 - 20.0 * colour.g()),
                0.0,
            ),
            rb: RigidBody::new(pos),
            mass: Mass::new(size * size * 100.0),
            ..default()
        });
}

// fn spawn_wall(commands: &mut Commands, width: f32, height: f32, pos: Point<f32>) {
//     println!("Spawning wall at {:?}", pos);
//     commands.spawn_bundle(RigidBodyBundle {
//         collider: Collider::rect(width, height),
//         rb: RigidBody::new(pos),
//         mass: Mass::new(0.0),
//         ..default()
//     });
// }

// spawn_wall(
//     &mut commands,
//     SCREEN_WIDTH / SCALE,
//     2.0,
//     Point2::new(SCREEN_WIDTH / SCALE / 2.0, SCREEN_HEIGHT / SCALE),
// );
// spawn_wall(
//     &mut commands,
//     SCREEN_WIDTH / SCALE,
//     2.0,
//     Point2::new(SCREEN_WIDTH / SCALE / 2.0, 0.0),
// );
// spawn_wall(
//     &mut commands,
//     2.0,
//     SCREEN_HEIGHT / SCALE,
//     Point2::new(SCREEN_WIDTH / SCALE, SCREEN_HEIGHT / SCALE / 2.0),
// );
// spawn_wall(
//     &mut commands,
//     2.0,
//     SCREEN_HEIGHT / SCALE,
//     Point2::new(0.0, SCREEN_HEIGHT / SCALE / 2.0),
// );

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorParams, WorldInspectorPlugin};
use fishics::{components::*, resources::*, SimpleFishicsPlugin, bundles::RigidBodyBundle};
use leafwing_input_manager::prelude::*;
use prima::prelude::*;
use torus_common::player::{bundles::*, components::*, PlayerPlugin};
use torus_common::physics::{PhysicsPlugin as TorusPhysicsPlugin, components::Drag};

const SCALE: f32 = 20.0;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Actionlike, Clone)]
enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
    Focus,
    Action,
}

#[derive(Component)]
struct MainCamera;

#[derive(Default)]
struct MousePosition(Vec2);

fn main() {
    // Debug settings
    let mut log_setting = LogSettings::default();
    log_setting.level = Level::WARN;

    // Build the app
    let mut app = App::new();

    let mut config = FishicsConfig::default();
    config.scale = SCALE;

    let window = WindowDescriptor {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        title: "Torus client".to_string(),
        ..Default::default()
    };

    // Insert resources
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.12)))
        .insert_resource(window)
        .insert_resource(log_setting)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            highlight_changes: false,
            ..Default::default()
        });

    // Add main plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<Resources>::new())
        .add_plugins(SimpleFishicsPlugin::default())
        .add_plugin(PlayerPlugin {
            ..Default::default()
        })
        .add_plugin(TorusPhysicsPlugin::default());

    // Input handling
    let mut input_map = InputMap::default();
    input_map.insert(PlayerAction::Left, KeyCode::A);
    input_map.insert(PlayerAction::Right, KeyCode::D);
    input_map.insert(PlayerAction::Up, KeyCode::W);
    input_map.insert(PlayerAction::Down, KeyCode::S);
    input_map.insert(PlayerAction::Focus, MouseButton::Right);
    input_map.insert(PlayerAction::Action, MouseButton::Left);

    app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(input_map)
        .insert_resource(MousePosition::default())
        .insert_resource(config);

    // Systems
    app.add_startup_system(setup)
        .add_system(cursor_system)
        .add_system(input_system)
        .add_system(bounds_system);

    // Run the app
    app.run();
}

fn setup(mut commands: Commands, mut mats: ResMut<Assets<PhysicsMaterial>>) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let basic_material = mats.add(PhysicsMaterial::new(0.8));

    let player = PlayerBundle::new("Player 1".to_string(), Point::new(0.0, 0.0), basic_material.clone());
    commands.spawn_bundle(player).insert(Player::default()).insert(Drag::default());

    spawn_circle(&mut commands, Point::<f32>::new(6.0, 10.0), Color::rgb(0.0, 0.6, 0.15), 3.0, basic_material.clone());
    spawn_circle(&mut commands, Point::<f32>::new(-10.0, -10.0), Color::rgb(0.6, 0.1, 0.1), 1.0, basic_material.clone());
}

fn input_system(
    action_state: Res<ActionState<PlayerAction>>,
    mouse_state: Res<MousePosition>,
    mut query: Query<&mut Controller, With<Player>>,
) {
    let mut player_controller = query.single_mut();

    let mut x = 0.0f32;
    let mut y = 0.0f32;
    if action_state.pressed(PlayerAction::Left) {
        x -= 1.0;
    }
    if action_state.pressed(PlayerAction::Right) {
        x += 1.0;
    }
    if action_state.pressed(PlayerAction::Up) {
        y += 1.0;
    }
    if action_state.pressed(PlayerAction::Down) {
        y -= 1.0;
    }

    player_controller.movement = if x.abs() + y.abs() <= 1.0 {
        Vec2::new(x, y)
    } else {
        Vec2::new(x, y).normalize()
    };

    player_controller.target = if action_state.pressed(PlayerAction::Focus) {
         Some(mouse_state.0)
    } else {
        None
    };
}

fn cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut data: ResMut<MousePosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        data.0 = world_pos;
    }
}



fn spawn_circle(
    commands: &mut Commands,
    pos: Point<f32>,
    colour: Color,
    radius: f32,
    properties: Handle<PhysicsMaterial>,
) {
    commands.spawn_bundle(RigidBodyBundle {
        collider: Collider::circle(radius),
        rb: RigidBody::new(pos),
        mass: Mass::new(radius * radius * std::f32::consts::PI * 100.0),
        render: ColliderRender { colour },
        properties,
        ..default()
    });
}

fn bounds_system(mut cubes: Query<(&RigidBody, &mut Velocity)>) {
    for (rb, mut vel) in cubes.iter_mut() {
        let x = rb.position().x;
        let y = rb.position().y;

        let o: Point<f32> = Point::zero();
        let a = rb.position();
        let b = a + vel.linear();

        if o.distance_squared(&a) > o.distance_squared(&b) {
            // getting closer to origin so do not warp
            continue;
        }

        let v = vel.linear();

        // Warp
        if x > SCREEN_WIDTH / SCALE / 2.0 {
            vel.set_linear(Vector::new(-v.x, v.y));
        } else if x < -SCREEN_WIDTH / SCALE / 2.0 {
            vel.set_linear(Vector::new(-v.x, v.y));
        }
        if y > SCREEN_HEIGHT / SCALE / 2.0 {
            vel.set_linear(Vector::new(v.x, -v.y));
        } else if y < -SCREEN_HEIGHT / SCALE / 2.0 {
            vel.set_linear(Vector::new(v.x, -v.y));
        }
    }
}

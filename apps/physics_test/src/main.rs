use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use bevy_inspector_egui::{
    InspectorPlugin, RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin,
};
use fishics::{bundles::*, components::*, resources::*, FishicsPlugin};
use prima::prelude::*;
use torus_common::{
    items::{bundles::*, Item, ItemContainer, ItemSystem, ItemVolume},
    Descriptor,
};

const SCALE: f32 = 20.0;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

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
        .add_plugin(FishicsPlugin {
            config,
            ..Default::default()
        });

    // Insert assets
    app.add_asset::<Item>();

    // Inspectable components
    app.register_inspectable::<ItemContainer>();
    app.register_inspectable::<ItemSystem>();
    app.register_inspectable::<ItemVolume>();

    // Systems
    app.add_startup_system(setup);
    app.add_startup_system(item_test);
    app.add_system(reset_cubes);

    // Run the app
    app.run();
}

fn item_test(mut commands: Commands, mut items: ResMut<Assets<Item>>) {
    let bananna = Item::new(1, Descriptor::named("Bananna", "Banan"));
    let bag = Item::new(2, Descriptor::named("Bag", "Bag"));

    let bananna_handle = items.add(bananna);
    let bag_handle = items.add(bag);

    commands
        .spawn_bundle(ItemContainerBundle {
            name: Name::new("Bag"),
            item: bag_handle,
            volume: ItemVolume::new(32),
            container: ItemContainer::new(10),
        })
        .with_children(|parent| {
            parent.spawn_bundle(ItemBundle {
                item: bananna_handle.clone(),
                name: Name::new("Bananna 1"),
                volume: ItemVolume::new(2),
            });
            parent.spawn_bundle(ItemBundle {
                item: bananna_handle.clone(),
                name: Name::new("Bananna 2"),
                volume: ItemVolume::new(2),
            });
        });
}

fn setup(mut commands: Commands, mut mats: ResMut<Assets<PhysicsMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let basic_material = mats.add(PhysicsMaterial::bouncy());

    spawn_circle(
        &mut commands,
        Point::new(-12.0, 0.0),
        Color::rgb(0.7, 0.5, 0.2),
        5.0,
        basic_material.clone(),
    );
    spawn_cube(
        &mut commands,
        Point::new(12.0, -2.0),
        Color::rgb(0.1, 0.5, 0.7),
        (3.0, 6.0),
        basic_material.clone(),
    );
}

fn reset_cubes(mut cubes: Query<(&RigidBody, &mut Velocity)>) {
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

fn spawn_cube(
    commands: &mut Commands,
    pos: Point<f32>,
    colour: Color,
    size: (f32, f32),
    properties: Handle<PhysicsMaterial>,
) {
    commands.spawn_bundle(RigidBodyBundle {
        collider: Collider::rect(size.0, size.1),
        velocity: Velocity::new(
            Vec2::new(
                10.0 - 20.0 * (1. - colour.r()),
                10.0 - 20.0 * (1. - colour.g()),
            ),
            0.0,
        ),
        rb: RigidBody::new(pos),
        mass: Mass::new(size.0 * size.1 * 100.0),
        render: ColliderRender { colour },
        properties,
        ..default()
    });
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
        velocity: Velocity::new(
            Vec2::new(
                10.0 - 20.0 * (1. - colour.r()),
                10.0 - 20.0 * (1. - colour.g()),
            ),
            0.0,
        ),
        rb: RigidBody::new(pos),
        mass: Mass::new(radius * radius * std::f32::consts::PI * 100.0),
        render: ColliderRender { colour },
        properties,
        ..default()
    });
}

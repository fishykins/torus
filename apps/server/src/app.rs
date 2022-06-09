use std::time::Duration;
use crate::{net::new_renet_server, resources::Lobby};
use bevy::{prelude::*, log::{LogSettings, Level, LogPlugin}, app::{ScheduleRunnerSettings, ScheduleRunnerPlugin}, diagnostic::DiagnosticsPlugin, asset::AssetPlugin};
use bevy_renet::RenetServerPlugin;
use fishics::{FishicsCorePlugin, FishicsLocalityPlugin, network::LocalNet};
use torus_core::entities::SpawnPool;
use super::systems::*;

pub fn torus_app() {
    let mut app = App::new();

    let mut log_setting = LogSettings::default();
    log_setting.level = Level::INFO;

    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 144.0,
    )))
    .insert_resource(log_setting)
    .insert_resource(new_renet_server())
    .insert_resource(Lobby::new())
    .insert_resource(SpawnPool::new());

    app.add_plugins(MinimalPlugins)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(RenetServerPlugin)
        .add_plugin(FishicsCorePlugin::headless())
        .add_plugin(FishicsLocalityPlugin::<LocalNet>::new(true, false, false));

    app.add_system(connections_handler)
        .add_system(net_event_handler)
        .add_system(physics_broadcast_system)
        .add_system(request_handler);


    app.run();
}
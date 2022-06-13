use super::systems::*;
use crate::{net::new_renet_server, resources::NetMap};
use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    asset::AssetPlugin,
    diagnostic::DiagnosticsPlugin,
    log::{Level, LogPlugin, LogSettings},
    prelude::*,
};
use bevy_renet::RenetServerPlugin;
use fishics::{network::LocalNet, FishicsCorePlugin, FishicsLocalityPlugin};
use std::time::Duration;
use torus_core::resources::SpawnPool;

pub fn torus_app() {
    let mut app = App::new();

    let mut log_setting = LogSettings::default();
    log_setting.level = Level::INFO;
    let fps = Duration::from_secs_f64(1.0 / 144.0);

    app.insert_resource(ScheduleRunnerSettings::run_loop(fps))
        .insert_resource(log_setting)
        .insert_resource(new_renet_server())
        .insert_resource(NetMap::new())
        .insert_resource(SpawnPool::new());

    app.add_plugins(MinimalPlugins)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(RenetServerPlugin)
        .add_plugin(FishicsCorePlugin::headless())
        .add_plugin(FishicsLocalityPlugin::<LocalNet>::new(true, false, false));

    app.add_system(lobby_manager)
        .add_system(net_event_handler)
        .add_system(physics_broadcast_system)
        .add_system(request_handler);

    app.run();
}

use bevy::{
    ecs::schedule::ShouldRun,
    log::{Level, LogSettings},
    prelude::*,
};
use bevy_renet::{
    renet::{ConnectToken, RenetClient, RenetConnectionConfig, NETCODE_KEY_BYTES},
    run_if_client_conected, RenetClientPlugin,
};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};
use torus_network::{packets::*, CHANNEL_PHYSICS, CHANNEL_REQUEST};
use torus_common::components::Agent;

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"bilbo is a massive massive goon."; // 32-bytes
const PROTOCOL_ID: u64 = 1;

struct Player(u64);

struct NetMap(HashMap<Agent, Entity>);

fn main() {
    let mut app = App::new();

    let mut log_setting = LogSettings::default();
    log_setting.level = Level::INFO;

    app.insert_resource(log_setting)
        .insert_resource(new_renet_client())
        .insert_resource(NetMap(HashMap::new()))
        .insert_resource(PhysicsPackets::new());

    app.add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin);

    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(run_if_client_conected)
            .with_system(receive_messages_system)
            .with_system(map_physics_packet),
    );

    app.add_system(get_player_data.with_run_criteria(run_if_no_player));

    app.run();
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    // This connect token should come from another system, NOT generated from the client.
    // Usually from a matchmaking system
    // The client should not have access to the PRIVATE_KEY from the server.
    let token = ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        15,
        vec![server_addr],
        None,
        PRIVATE_KEY,
    )
    .unwrap();
    RenetClient::new(current_time, socket, client_id, token, connection_config).unwrap()
}

fn run_if_no_player(player: Option<Res<Player>>) -> ShouldRun {
    if player.is_none() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn get_player_data(mut client: ResMut<RenetClient>) {
    let packet = RequestPacket::RequestPlayerInfo;
    client.send_message(CHANNEL_REQUEST, packet.encode());
}

fn receive_messages_system(mut client: ResMut<RenetClient>, mut commands: Commands) {
    while let Some(data) = client.receive_message(CHANNEL_REQUEST) {
        let packet = RequestPacket::decode(&data);
        println!("{:?}", packet);
        match packet {
            RequestPacket::PlayerInfo(agent_uuid) => {
                let player = Player(agent_uuid);
                commands.insert_resource(player);
                println!("Player has been allocated uuid {}.", agent_uuid);
            }
            _ => panic!("Unexpected response packet"),
        }
    }

    while let Some(data) = client.receive_message(CHANNEL_PHYSICS) {
        let physics_packet = PhysicsPackets::decode(&data);
        commands.insert_resource(physics_packet);
    }
}

fn map_physics_packet(
    _commands: Commands,
    net_map: ResMut<NetMap>,
    physics_packet: Res<PhysicsPackets>,
) {
    for (agent, _physics_packet) in physics_packet.iter() {
        if let Some(_entity) = net_map.0.get(&agent) {
            // We already have this object so just update it.
        } else {
            // We don't have this object so ask for a bit more info...
            println!("Requesting more info for agent {}", agent.id());
        }
    }
}

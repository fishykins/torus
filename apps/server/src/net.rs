use std::{net::UdpSocket, time::SystemTime};
use bevy_renet::renet::{RenetServer, RenetConnectionConfig, ServerConfig};
use crate::{PROTOCOL_ID, PRIVATE_KEY};

/// Constructs the core network server.
pub fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, *PRIVATE_KEY);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
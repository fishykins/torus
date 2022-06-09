use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use torus_common::components::Agent;
use torus_network::{CHANNEL_REQUEST, packets::{RequestPacket, Packet}};

use crate::resources::{Lobby, PlayerStatus};


/// Handles network requests received from clients.
pub fn request_handler(mut server: ResMut<RenetServer>, lobby: Res<Lobby>, agents: Query<&Agent>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(data) = server.receive_message(client_id, CHANNEL_REQUEST) {
            let packet = RequestPacket::decode(&data);
            match packet {
                RequestPacket::RequestPlayerInfo => {
                    // Have we spawned them yet?
                    if let Some(player) = lobby.get_player(client_id) {
                        match player.status {
                            PlayerStatus::Assigned(entity) => {
                                if let Ok(agent) = agents.get(entity) {
                                    let message = RequestPacket::PlayerInfo(agent.id());
                                    server.send_message(
                                        client_id,
                                        CHANNEL_REQUEST,
                                        message.encode(),
                                    );
                                }
                            }
                            _ => {}
                        }
                        // Send them their player info.
                    }
                }
                _ => panic!("Unknown request packet."),
            }
        }
    }
}

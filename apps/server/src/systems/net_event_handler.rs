use bevy::prelude::*;
use bevy_renet::renet::ServerEvent;

use crate::resources::Lobby;

/// Handles network events, such as connections and discoonections.
pub fn net_event_handler(mut server_events: EventReader<ServerEvent>, mut lobby: ResMut<Lobby>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Player {} connected.", id);
                lobby.add_player(*id);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Player {} disconnected.", id);
                lobby.remove_player(*id);
            }
        }
    }
}

use super::Packet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestPacket {
    /// Request player info from the server
    RequestPlayerInfo,
    /// Used for allocating a players main agent.
    PlayerInfo(u64),
}

impl Packet for RequestPacket {
    fn decode(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
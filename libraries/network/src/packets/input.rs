use super::Packet;
use serde::{Deserialize, Serialize};
use torus_common::components::Controller;

/// Contains positional/velocity data for an entity.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ControllerPacket {
    controller: Controller,
}

impl Packet for ControllerPacket {
    fn decode(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
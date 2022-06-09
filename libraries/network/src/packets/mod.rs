mod request;
mod input;
mod physics;

pub use request::*;
pub use input::*;
pub use physics::*;

use serde::Serialize;

pub trait Packet: Serialize {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn decode(data: &[u8]) -> Self;
}

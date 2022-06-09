mod resources;
mod systems;
mod app;
mod net;

use app::torus_app;
use bevy_renet::renet::NETCODE_KEY_BYTES;

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"bilbo is a massive massive goon."; // 32-bytes
const PROTOCOL_ID: u64 = 1;

fn main() {
    torus_app();
}



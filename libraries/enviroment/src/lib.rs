//! Game map specific environments and their components.
//! No game logic should be placed here, and all procedural generation should be handled in dedicated crates.

pub mod navigation;
pub mod mesh;
pub mod builders;

pub type DefaultIx = usize;
pub type DefaultFloat = f32;
pub type DefaultInt = i32;
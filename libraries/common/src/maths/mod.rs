use bevy_inspector_egui::Inspectable;

/// The amount of liquid a container can hold. Measured in metric units such as liters
#[derive(Debug, Clone, Inspectable, Default)]
pub struct Capacity(u32);

/// How much space an object takes up. Measured in cubic units such as cubic meters.
#[derive(Debug, Clone, Inspectable, Default)]
pub struct Volume(u32);


// ================================================================== //
// ================================================================== //

impl Capacity {
    pub fn new(capacity: u32) -> Self {
        Self(capacity)
    }
}

impl Volume {
    pub fn new(volume: u32) -> Self {
        Self(volume)
    }
}
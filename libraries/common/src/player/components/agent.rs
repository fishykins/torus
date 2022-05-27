use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// While [Component]s may be owned by different localities, this indicates the main representative of the [Entity].
/// This is ceremonial and does not explicitly do anything, acting more as a tag than a value.
#[derive(Component, Inspectable, Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Agent<L>
where
    L: Default + Clone + Default + Inspectable + Reflect,
{
    owner: L
}

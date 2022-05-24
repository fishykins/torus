use std::hash::Hash;

use bevy::reflect::TypeUuid;
use bevy_inspector_egui::Inspectable;
use torus_common::units::*;

use crate::{Descriptor, Tags};

/// A simple type that holds an item's unique identifier. This is a global asset,
/// since all items of the same type share this data!
#[derive(Debug, Clone, Inspectable, TypeUuid)]
#[uuid = "1c307d3b-14e0-4998-b54c-4bab0102aa83"]
pub struct Item {
    pub uid: u32,
    pub description: Descriptor,
    pub tags: Tags<ItemTag>,
}

/// Something that has a structured sub-set of items. Similar to a container, but more specific.
pub struct ItemSystem {}

/// Something that has capacity and can hold generic items.
pub struct ItemContainer {
    pub capacity: Capacity,
}

// ================================================================== //
// ================================================================== //
// ================================================================== //

#[derive(Debug, Clone, Inspectable)]
pub enum ItemTag {
    None,
    Weapon,
    Armor,
    Consumable,
    Tool,
    Material,
    Military,
    Civilian,
    Rare,
}

// ================================================================== //
// ================================================================== //
// ================================================================== //

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}
impl Eq for Item {}

impl Default for ItemTag {
    fn default() -> Self {
        ItemTag::None
    }
}

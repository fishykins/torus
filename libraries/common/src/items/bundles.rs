use crate::items::{Item, ItemSystem, ItemContainer, ItemVolume};
use bevy::prelude::{Bundle, Handle};
use bevy::core::Name;


#[derive(Bundle, Default)]
pub struct ItemBundle {
    pub name: Name,
    pub item: Handle<Item>,
    pub volume: ItemVolume,
}

#[derive(Bundle, Default)]
pub struct ItemSystemBundle {
    pub name: Name,
    pub item: Handle<Item>,
    pub system: ItemSystem,
    pub volume: ItemVolume,
}

#[derive(Bundle, Default)]
pub struct ItemContainerBundle {
    pub name: Name,
    pub item: Handle<Item>,
    pub container: ItemContainer,
    pub volume: ItemVolume,
}
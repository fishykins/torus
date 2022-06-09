use bevy::prelude::Component;
pub mod components;
pub mod items;
pub mod maths;
pub mod meta;

// ================================================================== //
// ================================================================== //

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Inspectable, PartialEq, Serialize, Deserialize)]
pub struct Descriptor {
    pub title_long: String,
    pub title_short: String,
    pub description: String,
    pub abbreviation: String,
}

#[derive(Debug, Clone, Default, Inspectable, Serialize, Deserialize)]
pub struct Tags<T>
where
    T: Default,
{
    pub tags: Vec<T>,
}

impl Descriptor {
    pub fn new(long: String, short: String, description: String) -> Self {
        Self {
            title_long: long,
            title_short: short,
            description,
            abbreviation: String::new(),
        }
    }

    pub fn named(long: &str, short: &str) -> Self {
        Self {
            title_long: long.to_string(),
            title_short: short.to_string(),
            description: String::new(),
            abbreviation: String::new(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
}

impl<T> Tags<T>
where
    T: Default + Clone + PartialEq,
{
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }

    pub fn with_tag(mut self, tag: T) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_tags(mut self, tags: Vec<T>) -> Self {
        self.tags.extend(tags);
        self
    }
}

impl<T> Iterator for Tags<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.tags.pop()
    }
}

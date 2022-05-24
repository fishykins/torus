//! Meta data associated with world item types, peoples, and other things. Treat this as a library of all the things you can find in the game/universe.
//! Where possible, types are kept agnostic in favour of config files being used to define specific things.

pub mod factions;
pub mod weapon;
pub mod lists;
pub mod items;

// ================================================================== //
// ================================================================== //

use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Default, Inspectable)]
pub struct Descriptor {
    pub title_long: String,
    pub title_short: String,
    pub description: String,
}

#[derive(Debug, Clone, Default, Inspectable)]
pub struct Tags<T> where T: Default {
    pub tags: Vec<T>,
}

impl Descriptor {
    pub fn new(long: String, short: String, description: String) -> Self {
        Self {
            title_long: long,
            title_short: short,
            description,
        }
    }

    pub fn named(long: &str, short: &str) -> Self {
        Self {
            title_long: long.to_string(),
            title_short: short.to_string(),
            description: "".to_string(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
}

impl<T> Tags<T> where T: Default + Clone {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
        }
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

impl<T> Iterator for Tags<T> where T: Default + Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.tags.pop()
    }
}
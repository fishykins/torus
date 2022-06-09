use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// This is the core component that holds an entities unique network id.
#[derive(
    Component,
    Inspectable,
    Default,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct Agent {
    id: u64,
    owner: Option<u64>,
}

impl Agent {
    pub fn new(owner: Option<u64>, id: u64) -> Self {
        Self { owner, id }
    }

    pub fn new_id(id: u64) -> Self {
        Self { owner: None, id }
    }

    pub fn with_owner(mut self, owner: u64) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn owner(&self) -> Option<u64> {
        self.owner
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(owner) = self.owner {
            write!(f, "Agent(id: {}, owner: {})", self.id, owner)
        } else {
            write!(f, "Agent(id: {})", self.id)
        }
    }
}

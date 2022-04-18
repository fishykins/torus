use crate::DefaultIx;
use serde::Deserialize;

/// A transitionary marker between two nodes.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Portal {
    pub a: PortalLink,
    pub b: PortalLink,
    pub flags: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PortalLink {
    None,
    Region(DefaultIx),
    Junction(DefaultIx),
    Portal(DefaultIx),
}
use serde::Deserialize;
use crate::DefaultIx;

/// A path between two junctions.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Path {
    /// Junction id
    pub a: DefaultIx,
    /// Junction id
    pub b: DefaultIx,
    pub flags: Vec<String>,
}

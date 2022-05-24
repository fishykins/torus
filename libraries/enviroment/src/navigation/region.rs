use prima::prelude::*;
use serde::Deserialize;

use crate::DefaultInt;

/// A non-specific area of the map.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Region {
    pub bounds: Aabr<DefaultInt>,
    pub flags: Vec<String>,
}

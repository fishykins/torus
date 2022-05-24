use crate::{DefaultInt, DefaultIx};
use prima::prelude::*;
use serde::Deserialize;

/// Node point with a position and connections.
#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
pub struct Junction {
    pub position: Point<DefaultInt>,
    pub shape: JunctionShape,
    pub style: JunctionStyle,
    pub flags: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Hash)]
pub enum JunctionShape {
    Default,
    Square,
    Octagon,
    Round,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Hash)]
pub enum JunctionStyle {
    Default,
    Wide,
    Normal,
    Narrow,
}

/// Something that can connect to a junction
#[derive(Debug, Deserialize, PartialEq, Clone, Eq, Hash)]
pub enum JunctionKey {
    None,
    Path(DefaultIx),
    Portal(DefaultIx),
}

impl Default for JunctionKey {
    fn default() -> Self {
        JunctionKey::None
    }
}

impl Default for JunctionStyle {
    fn default() -> Self {
        Self::Default
    }
}

impl Default for JunctionShape {
    fn default() -> Self {
        Self::Default
    }
}

use serde::{Serialize, Deserialize};
use crate::{Descriptor, Tags};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FactionType {
    Corporation,
    Government,
    Military,
    State,
    Independent,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FactionTag {
    Ammunition,
    Armour,
    Biological,
    Chemical,
    Civilian,
    Electronics,
    Explosives,
    Freight,
    Industrial,
    Logistics,
    Manufacturing,
    Materials,
    Medical,
    Military,
    Mining,
    Research,
    Ships,
    Tools,
    Weapons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Faction {
    pub uid: u16,
    pub descriptor: Descriptor,
    pub r#type: FactionType,
    pub tags: Tags<FactionTag>,
}

impl Default for FactionType {
    fn default() -> Self {
        FactionType::Corporation
    }
}

impl Default for FactionTag {
    fn default() -> Self {
        FactionTag::Civilian
    }
}
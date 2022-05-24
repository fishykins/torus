use serde::{Serialize, Deserialize};

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Faction {
    pub uid: u16,
    pub long_name: String,
    pub short_name: String,
    pub abrv: String,
    pub description: String,
    pub r#type: FactionType,
    pub tags: Vec<FactionTag>,
}

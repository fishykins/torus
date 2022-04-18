use super::{Junction, Path, Portal, Region};
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

/// The fundamental description of a map, containing all the junctions, regions, and paths.
/// Use an external crate to render the mesh with more complex geometry. The data structures are kept
/// simplistic in order to facilitate serialization and deserialization.
///
/// ## Note
/// This data structure ill be considered 'valid' by any code that uses it. Little to no checking will be done
/// to ensure that the data all lines up, and treating the data as mutable is not recommended.
/// In the future there may be a validator, but for the time being treat with caution.
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct NavMap {
    /// The pathways of the map. These are generally treated as a straight line, but this is ultimately determined by the renderer.
    paths: Vec<Path>,
    /// The junctions of the map, or node points by which everything else is connected.
    junctions: Vec<Junction>,
    /// The rooms of the map, or larger areas off-limits to the navigation pathways. Try not to let these intersect with paths!
    regions: Vec<Region>,
    /// Connections between junctions and regions. Typically, this would be a door!
    portals: Vec<Portal>,
}

impl NavMap {
    pub fn from_assets(map_name: &str) -> Result<Self, ron::Error> {
        let input_path = format!("../../assets/maps/{}.ron", map_name);
        let f = File::open(&input_path).expect("Failed opening file");
        let nav: Self = from_reader(f)?;
        Ok(nav)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_map_from_assets() {
        let nav = NavMap::from_assets("test_map").unwrap();
        assert_eq!(nav.paths.len(), 3);
        assert_eq!(nav.junctions.len(), 4);
        assert_eq!(nav.regions.len(), 1);
        assert_eq!(nav.portals.len(), 1);
    }
}
use bimap::BiMap;
use bevy::prelude::*;

type ClientId = u64;

#[derive(Clone)]
pub struct NetMap {
    teather: BiMap<ClientId, Entity>,
}

impl NetMap {
    pub fn new() -> Self {
        NetMap {
            teather: BiMap::new(),
        }
    }

    pub fn add(&mut self, client: ClientId, entity: Entity) {
        self.teather.insert(client, entity);
    }

    pub fn remove_client(&mut self, client: ClientId) {
        self.teather.remove_by_left(&client);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.teather.remove_by_right(&entity);
    }

    pub fn get_client(&self, entity: Entity) -> Option<ClientId> {
        let ret = self.teather.get_by_right(&entity);
        if ret.is_some() {
            return Some(*ret.unwrap())
        }
        None
    }

    pub fn get_entity(&self, client: ClientId) -> Option<Entity> {
        let ret = self.teather.get_by_left(&client);
        if ret.is_some() {
            return Some(ret.unwrap().clone())
        }
        None
    }
}
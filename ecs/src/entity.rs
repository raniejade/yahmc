use bit_set::BitSet;
use fxhash::FxHashMap;
use std::any::TypeId;
use std::default::Default;

use super::resource::{Fetch, FetchMut, Resources};
use super::system::SystemData;

pub type Entity = usize;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct EntityStorage {
    next_id: usize,
    alive: BitSet,
    limbo: Vec<usize>,
}

impl EntityStorage {
    pub fn create(&mut self) -> Entity {
        let id = if (!self.limbo.is_empty()) {
            self.limbo.remove(0)
        } else {
            self.next_id()
        };

        self.alive.insert(id);
        id as Entity
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.alive.contains(entity)
    }

    pub fn destroy(&mut self, entity: Entity) {
        assert!(self.is_alive(entity), "Can't destroy dead entity!");
        self.limbo.push(entity);
        self.alive.remove(entity);
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id = id + 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity() {
        let mut entity_storage = EntityStorage::new();
        let entity = entity_storage.create();
        assert!(entity_storage.is_alive(entity));
    }

    #[test]
    fn destroy_entity() {
        let mut entity_storage = EntityStorage::new();
        let entity = entity_storage.create();
        entity_storage.destroy(entity);
        assert!(!entity_storage.is_alive(entity));
    }

    #[test]
    #[should_panic]
    fn destroy_entity_dead() {
        let mut entity_storage = EntityStorage::new();
        let entity = entity_storage.create();
        entity_storage.destroy(entity);
        // some time later
        entity_storage.destroy(entity);
    }

    #[test]
    fn re_use_dead_entity() {
        let mut entity_storage = EntityStorage::new();
        let entity = entity_storage.create();
        entity_storage.destroy(entity);

        let new_entity = entity_storage.create();
        assert_eq!(entity, new_entity);
    }
}

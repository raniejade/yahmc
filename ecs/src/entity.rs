use bit_set::BitSet;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, MutexGuard, LockResult};

use super::resource::Fetch;

pub type Entity = usize;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct EntityStorage {
    next_id: Arc<Mutex<RefCell<usize>>>,
    alive: Arc<Mutex<RefCell<BitSet>>>,
    limbo: Arc<Mutex<RefCell<Vec<usize>>>>,
}

pub type Entities<'a> = Fetch<'a, EntityStorage>;

const LOCK_POISOINED: &str = "Lock is poisoned!";

impl EntityStorage {
    pub fn create(&self) -> Entity {
        let id = if !self.is_limbo_empty() {
            // self.limbo.remove(0);
            self.unlock_mut(self.limbo.lock(), |limbo: &mut Vec<usize>| {
                limbo.remove(0)
            })
        } else {
            self.next_id()
        };

        self.unlock_mut(self.alive.lock(), |alive: &mut BitSet| {
            alive.insert(id)
        });
        id as Entity
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.unlock(self.alive.lock(), |alive: &BitSet| {
            alive.contains(entity)
        })
    }

    pub fn destroy(&self, entity: Entity) {
        assert!(self.is_alive(entity), "Can't destroy dead entity!");
        self.unlock_mut(self.limbo.lock(), |limbo: &mut Vec<usize>| {
            limbo.push(entity)
        });
        self.unlock_mut(self.alive.lock(), |alive: &mut BitSet| {
            alive.remove(entity)
        });
    }

    fn is_limbo_empty(&self) -> bool {
        self.unlock(self.limbo.lock(), |limbo: & Vec<usize>| {
            limbo.is_empty()
        })
    }

    fn next_id(&self) -> usize {
        self.unlock_mut(self.next_id.lock(), |current: &mut usize| {
            let id = *current;
            *current = id + 1;
            id
        })
    }

    fn unlock<T, R, F>(&self, lock: LockResult<MutexGuard<RefCell<T>>>, cb: F) -> R
    where
        F: Fn(&T) -> R
    {
        let value = lock.expect(LOCK_POISOINED);
        let actual_value = value.borrow();
        cb(&actual_value)
    }

    fn unlock_mut<T, R, F>(&self, lock: LockResult<MutexGuard<RefCell<T>>>, cb: F) -> R
    where
        F: Fn(&mut T) -> R
    {
        let value = lock.expect(LOCK_POISOINED);
        let mut actual_value = value.borrow_mut();
        cb(&mut actual_value)
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
    
    #[test]
    fn unique_ids() {
        let mut entity_storage = EntityStorage::new();
        let entity1 = entity_storage.create();
        let entity2 = entity_storage.create();
        assert_ne!(entity1, entity2);
    }
}

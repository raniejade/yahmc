pub mod storage;

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use self::storage::{RawStorage, MaskedStorage};
use super::entity::{Entity, Entities};
use super::resource::{Fetch, FetchMut, Resources};
use super::system::SystemData;

pub trait Component: Any + Sized {
    type Storage: RawStorage<Self> + Any + Send + Sync;
}

trait EntityChecker {
    fn assert_alive(&self, entity: Entity);
}

impl EntityChecker for Entities {
    fn assert_alive(&self, entity: Entity) {
        if !self.is_alive(entity) {
            panic!("Entity {} is not alive.", entity);
        }
    }
}

pub struct Storage<'a, T, D>
where
    T: Component,
{
    entities: Fetch<'a, Entities>,
    data: D,
    phantom: PhantomData<T>,
}

pub type ReadStorage<'a, T> = Storage<'a, T, Fetch<'a, MaskedStorage<T>>>;

impl<'a, T, D> Storage<'a, T, D>
where
    T: Component,
    D: Deref<Target=MaskedStorage<T>>
{
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.assert_alive(entity);
        self.data.contains(entity)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.entities.assert_alive(entity);
        self.data.get(entity)
    }
}

impl<'a, T> SystemData<'a> for ReadStorage<'a, T>
where
    T: Component,
{
    fn fetch(res: &'a Resources) -> Self {
        let entities = res.fetch::<Entities>();
        let data = res.fetch::<MaskedStorage<T>>();
        Storage {
            entities,
            data,
            phantom: PhantomData,
        }
    }
}

pub type WriteStorage<'a, T> = Storage<'a, T, FetchMut<'a, MaskedStorage<T>>>;

impl<'a, T, D> Storage<'a, T, D>
where
    T: Component,
    D: DerefMut<Target=MaskedStorage<T>>
{

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.entities.assert_alive(entity);
        self.data.get_mut(entity)
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.entities.assert_alive(entity);
        self.data.insert(entity, component)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        self.entities.assert_alive(entity);
        self.data.remove(entity)
    }
}

impl<'a, T> SystemData<'a> for WriteStorage<'a, T>
where
    T: Component,
{
    fn fetch(res: &'a Resources) -> Self {
        let entities = res.fetch::<Entities>();
        let data = res.fetch_mut::<MaskedStorage<T>>();
        Storage {
            entities,
            data,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::storage::VecStorage;

    #[derive(Component)]
    #[Storage(VecStorage)]
    struct MyComponent;

    type MyReadStorage<'a> = ReadStorage<'a, MyComponent>;
    type MyWriteStorage<'a> = WriteStorage<'a, MyComponent>;

    #[test]
    #[should_panic]
    fn contains_dead_entity() {
        let mut resources = Resources::new();
        resources.add(Entities::new());
        resources.add(<MaskedStorage<MyComponent>>::new());
        let storage = MyReadStorage::fetch(&resources);
        storage.contains(0);
    }

    #[test]
    #[should_panic]
    fn get_dead_entity() {
        let mut resources = Resources::new();
        resources.add(Entities::new());
        resources.add(<MaskedStorage<MyComponent>>::new());
        let storage = MyReadStorage::fetch(&resources);
        storage.get(0);
    }

    #[test]
    #[should_panic]
    fn get_mut_dead_entity() {
        let mut resources = Resources::new();
        resources.add(Entities::new());
        resources.add(<MaskedStorage<MyComponent>>::new());
        let mut storage = MyWriteStorage::fetch(&resources);
        storage.get_mut(0);
    }

    #[test]
    #[should_panic]
    fn insert_dead_entity() {
        let mut resources = Resources::new();
        resources.add(Entities::new());
        resources.add(<MaskedStorage<MyComponent>>::new());
        let mut storage = MyWriteStorage::fetch(&resources);
        storage.insert(0, MyComponent);
    }

    #[test]
    #[should_panic]
    fn remove_dead_entity() {
        let mut resources = Resources::new();
        resources.add(Entities::new());
        resources.add(<MaskedStorage<MyComponent>>::new());
        let mut storage = MyWriteStorage::fetch(&resources);
        storage.remove(0);
    }
}
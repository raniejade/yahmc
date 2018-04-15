#[macro_use]
extern crate ecs_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate mopa;
extern crate fxhash;
extern crate bit_set;

mod resources;
mod system;
mod entity;
mod component;
mod storage;

use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct Storage<T, D>
where T: component::Component {
    data: D,
    phantom: PhantomData<T>
}

pub type ReadStorage<'a, T> = Storage<T, resources::Fetch<'a, storage::MaskedStorage<T>>>;

impl<T, D> Storage<T, D>
where T: component::Component,
      D: Deref<Target=storage::MaskedStorage<T>> {
    pub fn get(&self, entity: entity::Entity) -> Option<&T> {
        self.data.get(entity)
    }

    pub fn contains(&self, entity: entity::Entity) -> bool {
        self.data.contains(entity)
    }
}

impl<'a, T> system::SystemData<'a> for ReadStorage<'a, T>
where T: component::Component {
    fn fetch(res: &'a resources::Resources) -> Self {
        let data = res.fetch::<storage::MaskedStorage<T>>();
        Storage {
            data,
            phantom: PhantomData
        }
    }
}


pub type WriteStorage<'a, T> = Storage<T, resources::FetchMut<'a, storage::MaskedStorage<T>>>;

impl<T, D> Storage<T, D>
where T: component::Component,
      D: DerefMut<Target=storage::MaskedStorage<T>> {
    pub fn get_mut(&mut self, entity: entity::Entity) -> Option<&mut T> {
        self.data.get_mut(entity)
    }

    pub fn insert(&mut self, entity: entity::Entity, component: T) {
        self.data.insert(entity, component)
    }

    pub fn remove(&mut self, entity: entity::Entity) -> Option<T> {
        self.data.remove(entity)
    }
}

impl<'a, T> system::SystemData<'a> for WriteStorage<'a, T>
where T: component::Component {
    fn fetch(res: &'a resources::Resources) -> Self {
        let data = res.fetch_mut::<storage::MaskedStorage<T>>();
        Storage {
            data,
            phantom: PhantomData
        }
    }
}
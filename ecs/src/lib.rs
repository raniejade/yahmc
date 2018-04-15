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

use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use component::Component;
use component::storage::MaskedStorage;
use entity::{Entity, EntityStorage};
use resources::{Fetch, FetchMut, Resources};

pub struct Storage<T, D>
where T: Component {
    data: D,
    phantom: PhantomData<T>
}

pub type ReadStorage<'a, T> = Storage<T, Fetch<'a, MaskedStorage<T>>>;

impl<T, D> Storage<T, D>
where T: Component,
      D: Deref<Target=MaskedStorage<T>> {
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(entity)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.data.contains(entity)
    }
}

impl<'a, T> system::SystemData<'a> for ReadStorage<'a, T>
where T: Component {
    fn fetch(res: &'a Resources) -> Self {
        let data = res.fetch::<MaskedStorage<T>>();
        Storage {
            data,
            phantom: PhantomData
        }
    }
}

pub type WriteStorage<'a, T> = Storage<T, FetchMut<'a, MaskedStorage<T>>>;

impl<T, D> Storage<T, D>
where T: Component,
      D: DerefMut<Target=MaskedStorage<T>> {
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(entity)
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.data.insert(entity, component)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        self.data.remove(entity)
    }
}

impl<'a, T> system::SystemData<'a> for WriteStorage<'a, T>
where T: Component {
    fn fetch(res: &'a Resources) -> Self {
        let data = res.fetch_mut::<MaskedStorage<T>>();
        Storage {
            data,
            phantom: PhantomData
        }
    }
}

pub type Entities<'a> = resources::FetchMut<'a, EntityStorage>;

impl<'a> system::SystemData<'a> for Entities<'a> {
    fn fetch(res: &'a resources::Resources) -> Self {
        res.fetch_mut::<entity::EntityStorage>()
    }
}
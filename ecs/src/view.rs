
use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::component::Component;
use super::component::storage::MaskedStorage;
use super::entity::Entity;
use super::system::SystemData;
use super::resources::{Fetch, FetchMut, Resources};

pub struct View<T, D>
where T: Component {
    data: D,
    phantom: PhantomData<T>
}

pub type ReadView<'a, T> = View<T, Fetch<'a, MaskedStorage<T>>>;

impl<T, D> View<T, D>
where T: Component,
      D: Deref<Target=MaskedStorage<T>> {
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(entity)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.data.contains(entity)
    }
}

impl<'a, T> SystemData<'a> for ReadView<'a, T>
where T: Component {
    fn fetch(res: &'a Resources) -> Self {
        let data = res.fetch::<MaskedStorage<T>>();
        View {
            data,
            phantom: PhantomData
        }
    }
}

pub type WriteView<'a, T> = View<T, FetchMut<'a, MaskedStorage<T>>>;

impl<T, D> View<T, D>
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

impl<'a, T> SystemData<'a> for WriteView<'a, T>
where T: Component {
    fn fetch(res: &'a Resources) -> Self {
        let data = res.fetch_mut::<MaskedStorage<T>>();
        View {
            data,
            phantom: PhantomData
        }
    }
}
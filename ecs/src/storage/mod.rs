use bit_set::BitSet;
use bit_set::Iter;
use std::any::Any;
use std::default::Default;
use std::iter::Iterator;
use std::mem;

use super::component::Component;

pub type Index = usize;

pub trait RawStorage<T: Component>: Default + Sized {
    fn get(&self, index: Index) -> &T;
    fn get_mut(&self, index: Index) -> &mut T;
    fn insert(&mut self, index: Index, component: T);
    fn remove(&mut self, index: Index) -> T;
}

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct MaskedStorage<T: Component>(BitSet, T::Storage);

impl<T: Component> MaskedStorage<T> {
    pub fn new() -> MaskedStorage<T> {
        Default::default()
    }

    pub fn contains(&self, index: Index) -> bool {
        self.0.contains(index)
    }

    pub fn get(&self, index: Index) -> Option<&T> {
        if self.contains(index) {
            Some(self.1.get(index))
        } else {
            None
        }
    }

    pub fn get_mut(&self, index: Index) -> Option<&mut T> {
        if self.contains(index) {
            Some(self.1.get_mut(index))
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: Index, mut component: T) {
        if self.contains(index) {
            mem::swap(&mut component, { self.1.get_mut(index) })
        } else {
            self.0.insert(index);
            self.1.insert(index, component);
        }
    }

    pub fn remove(&mut self, index: Index) -> Option<T> {
        if self.contains(index) {
            Some(self.1.remove(index))
        } else {
            None
        }
    }
}
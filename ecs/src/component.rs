use fxhash::FxHashMap;
use mopa::Any;
use std::any::TypeId;
use std::default::Default;
use std::cell::{Cell, RefCell};

pub(crate) type ComponentId = u32;

pub trait Component: Any + Send + Sync { }

mopafy!(Component);

#[derive(Default)]
pub(crate) struct ComponentManager(Cell<u32>, FxHashMap<TypeId, ComponentId>);

impl ComponentManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register<T>(&mut self) -> ComponentId
    where T: Component {
        use std::collections::hash_map::Entry;

        let entry;
        {
            entry = self.1.entry(TypeId::of::<T>());
        }

        if let Entry::Vacant(e) = entry {
            let id = self.0.get();
            // compute the next id
            self.0.set(id + 1);
            e.insert(id);
            id
        } else {
            panic!("Component already registered!");
        }
    }

    pub fn get<T>(&self) -> Option<&ComponentId>
    where T: 'static + Component {
        self.1.get(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct C(u32);

    #[test]
    fn register() {
        let mut manager = ComponentManager::new();
        manager.register::<C>();

        assert!(manager.get::<C>().is_some());
    }

    #[test]
    fn not_registered() {
        let manager = ComponentManager::new();

        assert!(manager.get::<C>().is_none());
    }

    #[test]
    #[should_panic]
    fn register_should_panic() {
        let mut manager = ComponentManager::new();
        manager.register::<C>();
        manager.register::<C>();
    }
}
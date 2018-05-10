use std::any::{Any, TypeId};
use std::default::Default;
use fxhash::FxHashMap;

use super::storage::Storage;

pub trait Component
where
    Self: Sized + Any + Send + Sync + Default
{
    type Storage: Storage<Self>;
}

pub(crate) type ComponentId = usize;

#[derive(Default)]
pub(crate) struct ComponentManager {
    current: usize,
    ids: FxHashMap<TypeId, ComponentId>
}

impl ComponentManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register<T>(&mut self) -> ComponentId
    where
        T: Component
    {
        use std::collections::hash_map::Entry;

        let entry = self.ids.entry(TypeId::of::<T>());

        if let Entry::Vacant(e) = entry {
            self.current = self.current + 1;
            e.insert(self.current);
            self.current
        } else {
            panic!("Component already registered!")
        }
    }

    pub fn id<T>(&self) -> ComponentId
    where
        T: Component
    {
        self.ids.get(&TypeId::of::<T>()).expect("Component not registered!")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::VecStorage;

    #[derive(Default)]
    struct MyComponent;
    impl Component for MyComponent {
        type Storage = VecStorage<Self>;
    }

    #[derive(Default)]
    struct AnotherComponent;
    impl Component for AnotherComponent {
        type Storage = VecStorage<Self>;
    }

    #[test]
    fn registered() {
        let mut manager = ComponentManager::new();
        let id = manager.register::<MyComponent>();
        assert_eq!(id, manager.id::<MyComponent>());
    }

    #[test]
    #[should_panic]
    fn already_registered() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<MyComponent>();
    }

    #[test]
    #[should_panic]
    fn not_registered() {
        let manager = ComponentManager::new();
        manager.id::<MyComponent>();
    }

    #[test]
    fn unique() {
        let mut manager = ComponentManager::new();
        assert_ne!(manager.register::<MyComponent>(), manager.register::<AnotherComponent>());
    }
}
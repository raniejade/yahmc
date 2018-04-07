use bit_set::BitSet;
use fxhash::FxHashMap;
use std::any::TypeId;

use super::component::{Component, ComponentId};

pub struct Entity<F: Fn(TypeId) -> ComponentId> {
    mask: BitSet,
    components: FxHashMap<ComponentId, Box<Component>>,
    component_mapper: F
}

impl<F: Fn(TypeId) -> ComponentId> Entity<F> {
    pub(crate) fn new(component_mapper: F) -> Self {
        Entity {
            mask: BitSet::new(),
            components: FxHashMap::default(),
            component_mapper
        }
    }

    pub fn add_component<T>(&mut self, component: T) 
    where T: Component {
        use std::collections::hash_map::Entry;
        let id = self.id_for::<T>();
        let entry = self.components.entry(id);

        if let Entry::Vacant(e) = entry {
            e.insert(Box::new(component));
        } else if let Entry::Occupied(mut o) = entry {
            o.insert(Box::new(component));
        }

        self.mask.insert(id as usize);
    }

    pub fn remove_component<T>(&mut self) -> Option<T> 
    where T: Component {
        let id = self.id_for::<T>();
        if (self.mask.contains(id as usize)) {
            self.mask.remove(id as usize);
            let value = self.components.remove(&id).map(|b| b.downcast::<T>())
                .expect("Failed to downcast component");
            return value.ok().map(|b| *b)
        }

        None
    }

    pub fn has_component<T>(&self) -> bool
    where T: Component {
        self.mask.contains(self.id_for::<T>() as usize)
    }

    fn id_for<T>(&self) -> ComponentId
    where T: Component {
        let mapper = &self.component_mapper;
        mapper(TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct C(u32);

    #[test]
    pub fn add_component() {
        let mut entity = Entity::new(|c| { 0 });
        let c = C(0);

        entity.add_component(c);
        assert!(entity.has_component::<C>());
    }

    #[test]
    pub fn no_component() {
        let mut entity = Entity::new(|c| { 0 });
        let c = C(0);

        assert!(!entity.has_component::<C>());
    }

    #[test]
    pub fn remove_component() {
        let mut entity = Entity::new(|c| { 0 });
        let c = C(0);

        entity.add_component(c);
        let removed = entity.remove_component::<C>();
        assert!(removed.is_some());
        assert!(!entity.has_component::<C>());
    }

    #[test]
    pub fn remove_component_none() {
        let mut entity = Entity::new(|c| { 0 });
        let removed = entity.remove_component::<C>();
        assert!(removed.is_none());
    }
}
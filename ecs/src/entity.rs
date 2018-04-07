use bit_set::BitSet;
use fxhash::FxHashMap;
use std::any::TypeId;

use super::component::{Component, ComponentId};

pub struct Entity<'a, F: Fn(TypeId) -> ComponentId> {
    mask: BitSet,
    components: FxHashMap<ComponentId, Box<Component>>,
    component_mapper: F,
    mark_dirty: &'a mut FnMut()
}

impl<'a, F: Fn(TypeId) -> ComponentId> Entity<'a, F> {
    pub(crate) fn new(component_mapper: F, mark_dirty: &'a mut FnMut()) -> Self {
        Entity {
            mask: BitSet::new(),
            components: FxHashMap::default(),
            component_mapper: component_mapper,
            mark_dirty: mark_dirty
        }
    }

    pub fn add_component<T>(&mut self, component: T) 
    where T: Component {
        use std::collections::hash_map::Entry;
        let id = self.id_for::<T>();
        let mark_dirty = &mut self.mark_dirty;
        let entry = self.components.entry(id);

        if let Entry::Vacant(e) = entry {
            e.insert(Box::new(component));
            mark_dirty();
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
            let mark_dirty = &mut self.mark_dirty;
            mark_dirty();
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
    fn add_component() {
        let mut mark_dirty = || {};
        let mut entity = Entity::new(|c| { 0 }, &mut mark_dirty);
        let c = C(0);


        entity.add_component(c);
        assert!(entity.has_component::<C>());
    }

    #[test]
    fn no_component() {
        let mut mark_dirty = || {};
        let mut entity = Entity::new(|c| { 0 }, &mut mark_dirty);
        let c = C(0);

        assert!(!entity.has_component::<C>());
    }

    #[test]
    fn remove_component() {
        let mut mark_dirty = || {};
        let mut entity = Entity::new(|c| { 0 }, &mut mark_dirty);
        let c = C(0);

        entity.add_component(c);
        let removed = entity.remove_component::<C>();
        assert!(removed.is_some());
        assert!(!entity.has_component::<C>());
    }

    #[test]
    fn remove_component_none() {
        let mut mark_dirty = || {};
        let mut entity = Entity::new(|c| { 0 }, &mut mark_dirty);
        let removed = entity.remove_component::<C>();
        assert!(removed.is_none());
    }

    #[test]
    fn dirty_when_adding() {
        let mut dirty = false;
        {
            let mut mark_dirty = || dirty = true;
            let mut entity = Entity::new(|c| { 0 }, &mut mark_dirty);
            let c = C(0);


            entity.add_component(c);
        }
        assert!(dirty)
    }

    #[test]
    fn dirty_when_removing() {
        let mut dirty = 0;
        {
            let mut mark_dirty = || dirty += 1;
            let mut entity = Entity::new(|c| { 0 }, &mut mark_dirty);
            let c = C(0);

            entity.add_component(c);
            entity.remove_component::<C>();
        }
        assert_eq!(2, dirty);
    }
} 
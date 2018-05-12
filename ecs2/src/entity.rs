use bit_set::BitSet;
use fxhash::FxHashMap;
use std::default::Default;

use aspect::{Aspect, Matcher};
use component::{Component, ComponentId, ComponentManager};

pub type Entity = usize;

pub struct EntityEditor<'a> {
    pub entity: Entity,
    entity_manager: &'a mut EntityManager,
}

impl<'a> EntityEditor<'a> {
    fn new(entity: Entity, entity_manager: &'a mut EntityManager) -> Self {
        EntityEditor {
            entity,
            entity_manager,
        }
    }

    pub fn add<T: Component>(&mut self, component: T) {
        self.entity_manager.add(self.entity, component);
    }

    pub fn remove<T: Component>(&mut self) -> T {
        self.entity_manager.remove(self.entity)
    }

    pub fn contains<T: Component>(&self) -> bool {
        self.entity_manager.contains::<T>(self.entity)
    }

    pub fn get<T: Component>(&self) -> Option<&T> {
        self.entity_manager.get::<T>(self.entity)
    }

    pub fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.entity_manager.get_mut::<T>(self.entity)
    }

    pub fn destroy(mut self) {
        self.entity_manager.destroy(self.entity);
    }

    pub fn commit(mut self) {
        self.entity_manager.commit(self.entity);
    }
}

#[derive(Default)]
pub(crate) struct EntityManager {
    storage: EntityStorage,
    states: EntityStates,
    index: AspectIndex,
    pub component_manager: ComponentManager,
}

impl EntityManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create(&mut self) -> EntityEditor {
        let entity = self.storage.create();
        // fuck the borrow checker
        {
            let bits = self.states.get(entity, true);
            self.index.update(&self.component_manager, entity, bits);
        }
        self.editor(entity)
    }

    pub fn editor(&mut self, entity: Entity) -> EntityEditor {
        if !self.storage.is_alive(entity) {
            panic!("Entity {} not alived!", entity);
        }
        EntityEditor::new(entity, self)
    }

    pub fn entities<T: Aspect>(&self) -> Vec<Entity> {
        self.index.entities::<T>(&self.component_manager)
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.storage.is_alive(entity)
    }

    pub fn register<T: Aspect>(&mut self) {
        self.index.register::<T>(&self.component_manager)
    }

    // entity manipulation methods
    fn add<T: Component>(&mut self, entity: Entity, component: T) {
        unimplemented!();
    }

    fn remove<T: Component>(&mut self, entity: Entity) -> T {
        unimplemented!();
    }

    fn contains<T: Component>(&self, entity: Entity) -> bool {
        unimplemented!();
    }

    fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        unimplemented!();
    }

    fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        unimplemented!();
    }

    fn destroy(&mut self, entity: Entity) {
        self.storage.destroy(entity);
        // clear state
        self.states.get(entity, true);
        self.index.remove(&self.component_manager, entity);
    }

    fn commit(&mut self, entity: Entity) {
        let bits = self.states.get(entity, false);
        self.index.update(&self.component_manager, entity, bits);
    }
}

#[derive(Default)]
struct EntityStates {
    state: FxHashMap<Entity, BitSet>,
}

impl EntityStates {
    fn new() -> Self {
        Default::default()
    }

    fn get(&mut self, entity: Entity, reset: bool) -> &mut BitSet {
        if !self.state.contains_key(&entity) || reset {
            self.state.insert(entity, BitSet::new());
        }

        self.state.get_mut(&entity).expect("state not found!")
    }
}

#[derive(Default)]
struct AspectIndex {
    index: FxHashMap<Matcher, BitSet>,
}

impl AspectIndex {
    fn new() -> Self {
        Default::default()
    }

    fn register<T: Aspect>(&mut self, component_manager: &ComponentManager) {
        let matcher = Matcher::new::<T>(component_manager);
        // TODO: should we fail if it exists?
        if !self.index.contains_key(&matcher) {
            self.index.insert(matcher, BitSet::new());
        }
    }

    fn update(&mut self, component_manager: &ComponentManager, entity: Entity, bits: &BitSet) {
        for (matcher, entities) in self.index.iter_mut() {
            if entities.contains(entity) {
                if !matcher.check(component_manager, bits) {
                    entities.remove(entity);
                }
            } else {
                if matcher.check(component_manager, bits) {
                    entities.insert(entity);
                }
            }
        }
    }

    fn remove(&mut self, component_manager: &ComponentManager, entity: Entity) {
        for (_, entities) in self.index.iter_mut() {
            entities.remove(entity);
        }
    }

    fn entities<T: Aspect>(&self, component_manager: &ComponentManager) -> Vec<Entity> {
        let matcher = Matcher::new::<T>(component_manager);

        if self.index.contains_key(&matcher) {
            let mut result = Vec::new();
            let index = self.index.get(&matcher).unwrap();
            for entity in index.iter() {
                result.push(entity);
            }
            return result;
        }

        panic!("Aspect not registered!");
    }
}

#[derive(Default)]
struct EntityStorage {
    next_id: usize,
    alive: BitSet,
    limbo: Vec<Entity>,
}

impl EntityStorage {
    fn new() -> Self {
        Default::default()
    }

    fn is_alive(&self, entity: Entity) -> bool {
        self.alive.contains(entity)
    }

    fn create(&mut self) -> Entity {
        let id = if self.limbo.is_empty() {
            self.next_id()
        } else {
            self.limbo.remove(0)
        };

        self.alive.insert(id);

        id
    }

    fn destroy(&mut self, entity: Entity) {
        if self.is_alive(entity) {
            self.limbo.push(entity);
            self.alive.remove(entity);
        } else {
            panic!("Entity is not alive!");
        }
    }

    fn next_id(&mut self) -> Entity {
        self.next_id = self.next_id + 1;
        self.next_id
    }
}

trait ComponentSetter {
    fn set_bit(component_manager: &ComponentManager, bits: &mut BitSet);
    fn unset_bit(component_manager: &ComponentManager, bits: &mut BitSet);
}

impl<T> ComponentSetter for T
where
    T: Component,
{
    fn set_bit(component_manager: &ComponentManager, bits: &mut BitSet) {
        bits.insert(component_manager.id::<T>());
    }

    fn unset_bit(component_manager: &ComponentManager, bits: &mut BitSet) {
        bits.remove(component_manager.id::<T>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_storage_create_unique() {
        let mut storage = EntityStorage::new();
        assert_ne!(storage.create(), storage.create());
    }

    #[test]
    fn entity_storage_alive() {
        let mut storage = EntityStorage::new();
        let entity = storage.create();
        assert!(storage.is_alive(entity));
    }

    #[test]
    fn entity_storage_reuse_entity() {
        let mut storage = EntityStorage::new();
        let entity = storage.create();
        storage.destroy(entity);
        let new_entity = storage.create();
        assert_eq!(entity, new_entity);
    }

    #[test]
    fn entity_storage_destroy() {
        let mut storage = EntityStorage::new();
        let entity = storage.create();
        storage.destroy(entity);
        assert!(!storage.is_alive(entity));
    }

    #[test]
    #[should_panic]
    fn entity_storage_destroy_dead() {
        let mut storage = EntityStorage::new();
        storage.destroy(1);
    }

    #[test]
    fn entity_states_get_not_existing() {
        let mut states = EntityStates::new();
        assert_eq!(0, states.get(0, false).len());
    }

    #[test]
    fn entity_states_get_existing() {
        let mut states = EntityStates::new();
        states.get(0, false).insert(1);
        assert!(states.get(0, false).contains(1));
    }

    #[test]
    fn entity_states_get_existing_init() {
        let mut states = EntityStates::new();
        states.get(0, false).insert(1);
        assert!(!states.get(0, true).contains(1));
    }

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
    fn aspect_index_initially_empty() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<MyComponent>();
        component_manager.register::<AnotherComponent>();

        let mut index = AspectIndex::new();

        type MyAspect = (MyComponent, AnotherComponent);

        index.register::<MyAspect>(&component_manager);
        assert!(index.entities::<MyAspect>(&component_manager).is_empty());
    }

    #[test]
    #[should_panic]
    fn aspect_index_not_registered() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<MyComponent>();
        component_manager.register::<AnotherComponent>();

        let mut index = AspectIndex::new();

        type MyAspect = (MyComponent, AnotherComponent);

        index.entities::<MyAspect>(&component_manager);
    }

    #[test]
    fn aspect_index_insert() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<MyComponent>();
        component_manager.register::<AnotherComponent>();

        let mut index = AspectIndex::new();
        type MyAspect = (MyComponent, AnotherComponent);
        index.register::<MyAspect>(&component_manager);

        let entity = 1;
        let mut bits = BitSet::new();
        MyComponent::set_bit(&component_manager, &mut bits);
        AnotherComponent::set_bit(&component_manager, &mut bits);

        index.update(&component_manager, entity, &bits);

        assert!(
            index
                .entities::<MyAspect>(&component_manager)
                .contains(&entity)
        );
    }

    #[test]
    fn aspect_index_removal() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<MyComponent>();
        component_manager.register::<AnotherComponent>();

        let mut index = AspectIndex::new();
        type MyAspect = (MyComponent, AnotherComponent);
        index.register::<MyAspect>(&component_manager);

        let entity = 1;
        let mut bits = BitSet::new();
        // first pass
        MyComponent::set_bit(&component_manager, &mut bits);
        AnotherComponent::set_bit(&component_manager, &mut bits);
        index.update(&component_manager, entity, &bits);

        // sometime later
        AnotherComponent::unset_bit(&component_manager, &mut bits);
        index.update(&component_manager, entity, &bits);

        assert!(!index
            .entities::<MyAspect>(&component_manager)
            .contains(&entity));
    }

    #[test]
    fn aspect_index_explicit_removal() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<MyComponent>();
        component_manager.register::<AnotherComponent>();

        let mut index = AspectIndex::new();
        type MyAspect = (MyComponent, AnotherComponent);
        index.register::<MyAspect>(&component_manager);

        let entity = 1;
        let mut bits = BitSet::new();
        // first pass
        MyComponent::set_bit(&component_manager, &mut bits);
        AnotherComponent::set_bit(&component_manager, &mut bits);
        index.update(&component_manager, entity, &bits);

        // sometime later
        index.remove(&component_manager, entity);

        assert!(!index
            .entities::<MyAspect>(&component_manager)
            .contains(&entity));
    }

    #[test]
    fn entity_manager_existing() {
        let mut entity_manager = EntityManager::new();
        let entity = entity_manager.create().entity;

        assert_eq!(entity, entity_manager.editor(entity).entity);
    }

    #[test]
    #[should_panic]
    fn entity_manager_not_existing() {
        let mut entity_manager = EntityManager::new();
        entity_manager.editor(1);
    }
}

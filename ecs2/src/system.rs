use std::default::Default;
use std::time::Duration;

use aspect::Aspect;
use context::{Context, InternalContext};
use entity::Entity;

pub trait System {
    type Aspect: Aspect;

    fn process(&mut self, context: &mut impl Context, duration: Duration, entities: Vec<Entity>);
}

trait Executor {
    fn execute(&mut self, context: &mut InternalContext, duration: Duration);
}

impl<T, K> Executor for K
where
    T: Aspect,
    K: System<Aspect = T>,
{
    fn execute(&mut self, context: &mut InternalContext, duration: Duration) {
        let entities = context.get_entities::<T>();
        self.process(context, duration, entities)
    }
}

#[derive(Default)]
pub(crate) struct SystemDispatcher<'a> {
    systems: Vec<Box<Executor + 'a>>,
}

impl<'a> SystemDispatcher<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(&mut self, system: impl System + 'a) {
        self.systems.push(Box::new(system))
    }

    pub fn dispatch(&mut self, context: &mut InternalContext, duration: Duration) {
        for system in self.systems.iter_mut() {
            system.execute(context, duration);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::storage::VecStorage;
    use super::*;
    use aspect::{Aspect, Not};
    use component::{Component, ComponentManager};
    use entity::{Entity, EntityManager};

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

    struct MySystem;
    impl System for MySystem {
        type Aspect = (MyComponent, Not<AnotherComponent>);

        fn process(&mut self, context: &mut impl Context, duration: Duration, entities: Vec<Entity>) {
            assert!(!entities.is_empty());
            for entity in entities {
                let editor = context.editor(entity);
                assert!(editor.contains::<MyComponent>());
                assert!(!editor.contains::<AnotherComponent>());
            }
        }
    }

    struct PanickingSystem;
    impl System for PanickingSystem {
        type Aspect = (MyComponent, Not<AnotherComponent>);

        fn process(&mut self, context: &mut impl Context, duration: Duration, entities: Vec<Entity>) {
            panic!("For testing!");
        }
    }

    #[test]
    fn get_aspect() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<MyComponent>();
        component_manager.register::<AnotherComponent>();

        // all good as long as this compiles
        let (_req, _not) = (
            <MySystem as System>::Aspect::req(&component_manager),
            <MySystem as System>::Aspect::not(&component_manager),
        );
    }

    #[test]
    #[should_panic]
    fn dispatch() {
        let mut entity_manager = EntityManager::new();
        entity_manager.component_manager.register::<MyComponent>();
        entity_manager.component_manager.register::<AnotherComponent>();
        let mut context = InternalContext::new(&mut entity_manager);

        let mut dispatcher = SystemDispatcher::new();
        dispatcher.register(PanickingSystem);

        dispatcher.dispatch(&mut context, Duration::from_millis(100));
    }
}

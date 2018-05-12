use std::default::Default;
use std::time::Duration;

use aspect::Aspect;
use context::Context;
use entity::Entity;

pub trait System {
    type Aspect;

    fn process(&mut self, context: &mut Context, duration: Duration, entities: Vec<Entity>);
}

trait Executor {
    fn execute(&mut self, context: &mut Context, duration: Duration);
}

impl<T, K> Executor for K
where
    T: Aspect,
    K: System<Aspect=T>
{
    fn execute(&mut self, context: &mut Context, duration: Duration) {
        let entities = context.get_entities::<T>();
        self.process(context, duration, entities)
    }
}

#[derive(Default)]
pub(crate) struct SystemDispatcher<'a> {
    systems: Vec<Box<Executor + 'a>>
}

impl<'a> SystemDispatcher<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn register<T: Aspect>(&mut self, system: impl System<Aspect=T> + 'a) {
        self.systems.push(Box::new(system))
    }

    fn dispatch(&mut self, context: &mut Context, duration: Duration) {
        for system in self.systems.iter_mut() {
            system.execute(context, duration);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::storage::VecStorage;
    use aspect::{Aspect, Not};
    use component::{Component, ComponentManager};
    use entity::Entity;

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

        fn process(&mut self, context: &mut Context, duration: Duration, entities: Vec<Entity>) {}
    }

    #[test]
    fn get_aspect() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<AnotherComponent>();

        // all good as long as this compiles
        let (_req, _not) = (<MySystem as System>::Aspect::req(&manager),
                            <MySystem as System>::Aspect::not(&manager));
    }
}
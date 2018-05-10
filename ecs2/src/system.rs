use std::time::Duration;

use super::context::Context;
use super::entity::Entity;

pub trait System {
    type Aspect;

    fn process(&self, context: &mut Context, duration: Duration, entities: Vec<Entity>);
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

        fn process(&self, context: &mut Context, duration: Duration, entities: Vec<Entity>) {}
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
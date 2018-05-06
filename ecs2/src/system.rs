use std::time::Duration;

use super::aspect::Aspect;
use super::entity::Entity;

pub trait System {
    type Aspect;

    fn process(&self, duration: Duration, entities: Vec<Entity>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aspect::{Aspect, Not};
    use component::{Component, ComponentManager};
    use entity::Entity;

    struct MyComponent;
    impl Component for MyComponent {}

    struct AnotherComponent;
    impl Component for AnotherComponent {}

    struct MySystem;

    impl System for MySystem {
        type Aspect = (MyComponent, Not<AnotherComponent>);

        fn process(&self, duration: Duration, entities: Vec<Entity>) {}
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
extern crate ecs;
#[macro_use]
extern crate ecs_derive;

use ecs::component::storage::VecStorage;
use ecs::component::{Component, WriteStorage};
use ecs::entity::Entities;
use ecs::join::Join;
use ecs::system::System;
use ecs::{Dispatcher, World};

#[derive(Component)]
#[Storage(VecStorage)]
struct MyComponent(i32);

struct MySystem;

impl<'a> System<'a> for MySystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, MyComponent>);

    fn run(&mut self, (entities, mut components): Self::SystemData) {
        for (entity, mut my_component) in (entities, components).join() {
            // okay
        }
    }
}

#[test]
fn registered_component() {
    let mut world = World::new();
    world.register::<MyComponent>();
    let mut dispatcher = Dispatcher::new();
    dispatcher.register(MySystem);
    dispatcher.dispatch(&world);
}

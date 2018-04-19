extern crate ecs;
#[macro_use]
extern crate ecs_derive;

use ecs::component::storage::VecStorage;
use ecs::component::{Component, WriteStorage};
use ecs::entity::Entities;
use ecs::system::System;
use ecs::view::{ReadView, WriteView};
use ecs::{Dispatcher, World};

#[derive(Component)]
#[Storage(VecStorage)]
struct MyComponent(i32);

struct MySystem;

impl<'a> System<'a> for MySystem {
    type SystemData = (ReadView<'a, Entities>, WriteStorage<'a, MyComponent>);

    fn run(&mut self, (entities, mut components): Self::SystemData) {
        // nada
    }
}

#[test]
#[should_panic]
fn unregistered_component() {
    let mut world = World::new();
    let mut dispatcher = Dispatcher::new();
    dispatcher.register(MySystem);
    dispatcher.dispatch(&world);
}

#[test]
fn registered_component() {
    let mut world = World::new();
    world.register::<MyComponent>();
    let mut dispatcher = Dispatcher::new();
    dispatcher.register(MySystem);
    dispatcher.dispatch(&world);
}

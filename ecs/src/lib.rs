#[macro_use]
extern crate ecs_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate mopa;
extern crate bit_set;
extern crate fxhash;

pub mod component;
pub mod entity;
pub mod resource;
pub mod system;
pub mod view;

use component::Component;
use component::storage::MaskedStorage;
use entity::Entities;
use resource::Resources;
use system::{System, SystemData};


trait SystemRunner<'a> {
    fn run(&mut self, resources: &'a Resources);
}

impl<'a, T, S> SystemRunner<'a> for S
where
    T: SystemData<'a>,
    S: System<'a, SystemData=T>
{
    fn run(&mut self, resources: &'a Resources) {
        self.run(<T>::fetch(resources))
    }
}

pub struct World {
    pub(crate) resources: Resources
}

impl World {
    pub fn new() -> Self {
        let mut resources = Resources::new();
        resources.add(Entities::new());
        World { 
            resources
         }
    }

    pub fn register<T>(&mut self) -> &mut Self
    where
        T: Component
    {
        self.resources.add(<MaskedStorage<T>>::new());
        self
    }
}

// The only reason we have this type is because
// we can't do a self borrow.
pub struct Dispatcher<'a> {
    systems: Vec<Box<SystemRunner<'a> + 'a>>
}

impl<'a> Dispatcher<'a> {
    pub fn new() -> Self {
        Dispatcher {
            systems: Vec::new()
        }
    }

    pub fn register<T>(&mut self, system: T) -> &mut Self 
    where T: 'a + System<'a> {
        self.systems.push(Box::new(system));
        self
    }

    pub fn dispatch(&mut self, world: &'a World) {
        let systems = self.systems.iter_mut();
        for system in systems {
            system.run(&world.resources);
        }
    }
}
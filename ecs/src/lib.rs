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

pub struct World<'a> {
    resources: Resources,
    systems: Vec<Box<SystemRunner<'a> + 'a>>
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World { 
            resources: Resources::new(),
            systems: Vec::new()
         }
    }

    pub fn register_component<T>(&mut self) -> &mut Self
    where
        T: Component
    {
        self.resources.add(<MaskedStorage<T>>::new());
        self
    }

    pub fn register_system<T>(&mut self, system: T) -> &mut Self 
    where T: 'a + System<'a> {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run(&'a mut self) {
        let systems = self.systems.iter_mut();
        for system in systems {
            system.run(&self.resources);
        }
    }
}
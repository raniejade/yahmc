extern crate bit_set;
extern crate fxhash;

pub mod aspect;
pub mod component;
pub mod context;
pub mod entity;
pub mod storage;
pub mod system;

use std::default::Default;
use std::time::Duration;

use aspect::Aspect;
use component::Component;
use entity::EntityManager;
use system::{System, SystemDispatcher};

pub struct World<'a> {
    entity_manager: EntityManager,
    dispatcher: SystemDispatcher<'a>
}

impl<'a> World<'a> {
    fn new(entity_manager: EntityManager, dispatcher: SystemDispatcher<'a>) -> Self {
        World {
            entity_manager,
            dispatcher
        }
    }
}

pub struct WorldBuilder<'a> {
    entity_manager: EntityManager,
    dispatcher: SystemDispatcher<'a>
}

impl<'a> WorldBuilder<'a> {
    pub fn new() -> Self {
        WorldBuilder {
            entity_manager: Default::default(),
            dispatcher: Default::default()
        }
    }

    pub fn register_component<T: Component>(&mut self) {
        self.entity_manager.component_manager.register::<T>();
    }

    pub fn register_system(&mut self, system: impl System + 'a) {
        self.dispatcher.register(system)
    }

    pub fn build(self) -> World<'a> {
        World::new(self.entity_manager, self.dispatcher)
    }
}

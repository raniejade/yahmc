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

use component::Component;
use entity::EntityManager;
use system::System;

pub struct World {
    entity_manager: EntityManager,
}

impl World {
    fn new(entity_manager: EntityManager) -> Self {
        World { entity_manager }
    }
}

pub struct WorldBuilder {
    entity_manager: EntityManager,
}

impl WorldBuilder {
    pub fn new() -> Self {
        WorldBuilder {
            entity_manager: Default::default(),
        }
    }

    pub fn register_component<T: Component>(&mut self) {}

    pub fn register_system(&mut self, system: impl System) {}

    pub fn build(self) -> World {
        World::new(self.entity_manager)
    }
}

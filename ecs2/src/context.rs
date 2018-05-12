use aspect::Aspect;
use component::Component;
use entity::{Entity, EntityEditor};

pub struct Context {}

impl Context {
    pub fn create(&mut self) -> EntityEditor {
        unimplemented!();
    }

    pub fn editor(&mut self, entity: Entity) -> EntityEditor {
        unimplemented!();
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        unimplemented!();
    }

    pub(crate) fn get_entities<T: Aspect>(&self) -> Vec<Entity> {
        unimplemented!();
    }
}

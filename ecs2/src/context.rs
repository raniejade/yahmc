use entity::{Entity, EntityEditor};
use component::Component;

pub struct Context {

}

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
}
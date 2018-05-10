use entity::{Entity, EntityEditor};

pub struct Context {

}

impl Context {
    pub fn create(&mut self) -> Option<EntityEditor> {
        unimplemented!();
    }

    pub fn editor(&mut self, entity: Entity) -> Option<EntityEditor> {
        unimplemented!();
    }
}
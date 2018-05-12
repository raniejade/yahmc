use aspect::Aspect;
use component::Component;
use entity::{Entity, EntityEditor, EntityManager};

pub trait Context {
    fn create(&mut self) -> EntityEditor;
    fn editor(&mut self, entity: Entity) -> EntityEditor;
    fn is_alive(&self, entity: Entity) -> bool;
}

pub(crate) struct InternalContext<'a> {
    entity_manager: &'a mut EntityManager
}

impl<'a> InternalContext<'a> {
    pub fn new(entity_manager: &'a mut EntityManager) -> Self {
        InternalContext {
            entity_manager
        }
    }

    pub fn get_entities<T: Aspect>(&self) -> Vec<Entity> {
        self.entity_manager.entities::<T>()
    }
}

impl<'a> Context for InternalContext<'a> {
    fn create(&mut self) -> EntityEditor {
        self.entity_manager.create()
    }

    fn editor(&mut self, entity: Entity) -> EntityEditor {
        self.entity_manager.editor(entity)
    }

    fn is_alive(&self, entity: Entity) -> bool {
        self.entity_manager.is_alive(entity)
    }
}
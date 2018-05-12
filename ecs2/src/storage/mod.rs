use std::default::Default;

use component::Component;
use entity::Entity;

pub trait Storage {
    type Type: Component;
    fn get(&self, entity: Entity) -> &Self::Type;
    fn get_mut(&mut self, entity: Entity) -> &mut Self::Type;
    fn add(&mut self, entity: Entity, component: Self::Type);
    fn remove(&mut self, entity: Entity) -> Self::Type;
}

mod vec;

pub use self::vec::VecStorage;

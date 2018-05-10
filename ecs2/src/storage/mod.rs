use std::default::Default;

use super::entity::Entity;
use super::component::Component;

pub trait Storage<T>: Default
where
    T: Component
{
    fn get(&self, entity: Entity) -> &T;
    fn get_mut(&mut self, entity: Entity) -> &mut T;
    fn add(&mut self, entity: Entity, component: T);
    fn remove(&mut self, entity: Entity) -> T;
}

mod vec;

pub use self::vec::VecStorage;
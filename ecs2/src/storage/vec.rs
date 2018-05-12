use std::mem;

use super::*;

pub struct VecStorage<T: Component> {
    components: Vec<T>,
}

impl<T> VecStorage<T>
where
    T: Component,
{
    fn new() -> Self {
        Default::default()
    }
}

const GROWTH_FACTOR: usize = 10;

impl<T> Default for VecStorage<T>
where
    T: Component,
{
    fn default() -> Self {
        let mut components = Vec::with_capacity(GROWTH_FACTOR);
        unsafe {
            components.set_len(GROWTH_FACTOR);
        }

        VecStorage { components }
    }
}

impl<T> Storage<T> for VecStorage<T>
where
    T: Component,
{
    fn get(&self, entity: Entity) -> &T {
        &self.components[entity]
    }

    fn get_mut(&mut self, entity: Entity) -> &mut T {
        &mut self.components[entity]
    }

    fn add(&mut self, entity: Entity, component: T) {
        let index = entity;
        if self.components.len() <= index {
            unsafe {
                let size = index + GROWTH_FACTOR;
                self.components.reserve(size);
                self.components.set_len(size);
            }
        }

        mem::replace(&mut self.components[index], component);
    }

    fn remove(&mut self, entity: Entity) -> T {
        unsafe { mem::replace(&mut self.components[entity], mem::zeroed()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MyComponent(i32);
    impl Component for MyComponent {
        type Storage = VecStorage<Self>;
    }

    #[test]
    fn get_existing() {
        let entity = 1;
        let mut storage: VecStorage<MyComponent> = VecStorage::new();

        storage.add(entity, MyComponent(1));

        let component = storage.get(entity);
        assert_eq!(1, component.0);
    }

    #[test]
    fn get_not_existing() {
        let entity = 1;
        let mut storage: VecStorage<MyComponent> = VecStorage::new();
        // data maybe all zeroes
        storage.get(entity);
    }

    #[test]
    fn get_previously_existing() {
        let entity = 1;
        let mut storage: VecStorage<MyComponent> = VecStorage::new();

        storage.add(entity, MyComponent(1));
        storage.remove(entity);

        // data maybe all zeroes
        storage.get(entity);
    }
}

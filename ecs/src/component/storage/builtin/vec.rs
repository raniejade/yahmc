use super::*;

#[derive(Derivative)]
#[derivative(Default(new = "true", bound = ""))]
pub struct VecStorage<T: Component> {
    vec: Vec<T>
}

impl<T> RawStorage<T> for VecStorage<T>
where T: Component {
    fn get(&self, index: Index) -> &T {
        &self.vec[index]
    }

    fn contains(&self, index: Index) -> bool {
        match self.vec.get(index) {
            Some(_) => true,
            None => false
        }
    }

    fn get_mut(&mut self, index: Index) -> &mut T {
        &mut self.vec[index]
    }

    fn insert(&mut self, index: Index, component: T) {
        if self.vec.capacity() <= index {
            self.vec.reserve(index + 10)
        }

        self.vec.insert(index, component)
    }

    fn remove(&mut self, index: Index) -> T {
        self.vec.remove(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyComponent;
    impl Component for MyComponent {
        type Storage = VecStorage<Self>;
    }

    struct MyOtherComponent(i32);
    impl Component for MyOtherComponent {
        type Storage = VecStorage<Self>;
    }

    #[test]
    fn vec_storage_insert() {
        let mut storage: VecStorage<MyComponent> = VecStorage::new();
        let component = MyComponent;
        storage.insert(0, component);
        assert!(storage.contains(0));
    }

    #[test]
    fn vec_storage_remove() {
        let mut storage: VecStorage<MyComponent> = VecStorage::new();
        let component = MyComponent;
        storage.insert(0, component);
        storage.remove(0);
        assert!(!storage.contains(0));
    }

    #[test]
    fn vec_storage_get() {
        let mut storage: VecStorage<MyComponent> = VecStorage::new();
        let component = MyComponent;
        storage.insert(0, component);
        // should not panic
        storage.get(0);
    }

    #[test]
    #[should_panic]
    fn vec_storage_get_panic() {
        let mut storage: VecStorage<MyComponent> = VecStorage::new();
        storage.get(0);
    }

    #[test]
    fn vec_storage_get_mut() {
        let mut storage: VecStorage<MyOtherComponent> = VecStorage::new();
        let component = MyOtherComponent(0);
        storage.insert(0, component);
        {
            let mut mutable_component = storage.get_mut(0);
            mutable_component.0 = 100;
        }
        let updated_component = storage.get(0);
        assert_eq!(updated_component.0, 100);
    }

    #[test]
    #[should_panic]
    fn vec_storage_get_mut_panic() {
        let mut storage: VecStorage<MyComponent> = VecStorage::new();
        storage.get_mut(0);
    }
}
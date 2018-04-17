use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::entity::Entity;
use super::resource::{Fetch, FetchMut, Resource, Resources};
use super::system::SystemData;

pub struct View<R, D>
where
    R: Resource,
{
    data: D,
    phantom: PhantomData<R>,
}

pub type ReadView<'a, R> = View<R, Fetch<'a, R>>;

impl<'a, R> Deref for ReadView<'a, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &R {
        &self.data
    }
}

impl<'a, R> SystemData<'a> for ReadView<'a, R>
where
    R: Resource,
{
    fn fetch(res: &'a Resources) -> Self {
        let data = res.fetch::<R>();
        View {
            data,
            phantom: PhantomData,
        }
    }
}

pub type WriteView<'a, R> = View<R, FetchMut<'a, R>>;

impl<'a, R> Deref for WriteView<'a, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &R {
        &self.data
    }
}

impl<'a, R> DerefMut for WriteView<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut R {
        &mut self.data
    }
}

impl<'a, R> SystemData<'a> for WriteView<'a, R>
where
    R: Resource,
{
    fn fetch(res: &'a Resources) -> Self {
        let data = res.fetch_mut::<R>();
        View {
            data,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SomeResource(i32);
    impl SomeResource {
        pub fn new() -> Self {
            SomeResource(0)
        }

        pub fn get(&self) -> i32 {
            self.0
        }

        pub fn set(&mut self, value: i32) {
            self.0 = value
        }
    }

    struct AnotherResource;

    #[test]
    fn deref_coercion() {
        let mut resources = Resources::new();
        resources.add(SomeResource::new());
        let some_resource = <ReadView<SomeResource>>::fetch(&resources);
        assert_eq!(some_resource.get(), 0);
    }

    #[test]
    fn deref_mut_coercion() {
        let mut resources = Resources::new();
        resources.add(SomeResource::new());
        let mut some_resource = <WriteView<SomeResource>>::fetch(&resources);
        some_resource.set(1);
        assert_eq!(some_resource.get(), 1);
    }

    #[test]
    fn multiple() {
        let mut resources = Resources::new();
        resources.add(SomeResource::new());
        resources.add(AnotherResource);
        let some_resource =
            <(ReadView<SomeResource>, ReadView<AnotherResource>)>::fetch(&resources);
        assert_eq!(some_resource.0.get(), 0);
    }
}

use std::any::TypeId;
use std::cell::{Ref, RefMut, RefCell};
use std::default::Default;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;
use mopa::Any;

use super::system::SystemData;

const RESOURCE_NOT_FOUND: &str = "No resource with the given id";

pub trait Resource: Any + Send + Sync {}

mopafy!(Resource);

impl<T> Resource for T 
where T: Any + Send + Sync {
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(pub TypeId);

impl ResourceId {
    pub fn new<T: Resource>() -> Self {
        ResourceId(TypeId::of::<T>())
    }
}

pub struct Fetch<'a, T: 'a> {
    inner: Ref<'a, Box<Resource>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> SystemData<'a> for Fetch<'a, T> 
where T: Resource {
    fn fetch(res: &'a Resources) -> Self {
        res.fetch()
    }
}

impl<'a, T> Deref for Fetch<'a, T>
where T: Resource {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

pub struct FetchMut<'a, T: 'a> {
    inner: RefMut<'a, Box<Resource>>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T: 'a> SystemData<'a> for FetchMut<'a, T> 
where T: Resource {
    fn fetch(res: &'a Resources) -> Self {
        res.fetch_mut()
    }
}

impl<'a, T> Deref for FetchMut<'a, T>
where T: Resource {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
where T: Resource {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.inner.downcast_mut_unchecked() }
    }
}

#[derive(Default)]
pub struct Resources {
    resources: FxHashMap<ResourceId, RefCell<Box<Resource>>>
}

impl Resources {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<R>(&mut self, resource: R)
    where R: Resource {
        use std::collections::hash_map::Entry;

        let entry = self.resources.entry(ResourceId::new::<R>());

        if let Entry::Vacant(e) = entry {
            e.insert(RefCell::new(Box::new(resource)));
        } else {
            panic!("Resouce already exists!");
        }
    }

    pub fn has_value(&self, res_id: ResourceId) -> bool {
        self.resources.contains_key(&res_id)
    }

    pub fn fetch<T>(&self) -> Fetch<T>
    where
        T: Resource,
    {
        self.try_fetch().expect(RESOURCE_NOT_FOUND)
    }

    pub fn try_fetch<T>(&self) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>()).map(|r| {
            Fetch {
                inner: r.borrow(),
                phantom: PhantomData,
            }
        })
    }

    pub fn fetch_mut<T>(&self) -> FetchMut<T>
    where
        T: Resource,
    {
        self.try_fetch_mut().expect(RESOURCE_NOT_FOUND)
    }

    pub fn try_fetch_mut<T>(&self) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>()).map(|r| {
            FetchMut {
                inner: r.borrow_mut(),
                phantom: PhantomData,
            }
        })
    }

    fn try_fetch_internal(&self, id: TypeId) -> Option<&RefCell<Box<Resource>>> {
        self.resources.get(&ResourceId(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Res;
    struct AnotherRes;

    #[test]
    fn res_id() {
        assert_eq!(
            ResourceId::new::<Res>(),
            ResourceId(TypeId::of::<Res>(),)
        );
    }

    #[test]
    fn add() {
        let mut res = Resources::new();
        res.add(Res);

        assert!(res.has_value(ResourceId::new::<Res>()));
        assert!(!res.has_value(ResourceId::new::<AnotherRes>()));
    }

    #[test]
    fn fetch_uses_id() {
        let mut res = Resources::new();
        res.add(5i32);

        assert_eq!(*res.fetch::<i32>(), 5);
    }

    #[test]
    fn mutate() {
        let mut res = Resources::new();
        res.add(5i32);

        {
            *res.fetch_mut::<i32>() *= 2;
        }

        {
            assert_eq!(*res.fetch::<i32>(), 10);
        }
    }
}
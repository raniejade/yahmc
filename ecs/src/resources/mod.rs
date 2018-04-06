use std::any::TypeId;
use std::cell::{Ref, RefMut, RefCell};
use std::default::Default;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use fnv::FnvHashMap;
use mopa::Any;

const RESOURCE_NOT_FOUND: &str = "No resource with the given id";

pub trait Resource: Any + Send + Sync {}

mopafy!(Resource);

impl<T> Resource for T 
where T: Any + Send + Sync {
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(pub TypeId, pub usize);

impl ResourceId {
    pub fn new<T: Resource>() -> Self {
        Self::new_with_id::<T>(0)
    }

    pub fn new_with_id<T: Resource>(id: usize) -> Self {
        ResourceId(TypeId::of::<T>(), id)
    }
}

trait ResourceData {
    
}

pub struct Fetch<'a, T: 'a> {
    inner: Ref<'a, Box<Resource>>,
    phantom: PhantomData<&'a T>,
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
    resources: FnvHashMap<ResourceId, RefCell<Box<Resource>>>
}

impl Resources {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<R>(&mut self, resource: R)
    where R: Resource {
        self.add_with_id::<R>(resource, 0)
    }

    pub fn add_with_id<R>(&mut self, r: R, id: usize)
    where
        R: Resource,
    {
        use std::collections::hash_map::Entry;

        let entry = self.resources.entry(ResourceId::new_with_id::<R>(id));

        if let Entry::Vacant(e) = entry {
            e.insert(RefCell::new(Box::new(r)));
        } else {
            panic!("Tried to add a resource though it is already registered");
        }
    }

    pub fn has_value(&self, res_id: ResourceId) -> bool {
        self.resources.contains_key(&res_id)
    }

    pub fn fetch<T>(&self, id: usize) -> Fetch<T>
    where
        T: Resource,
    {
        self.try_fetch(id).expect(RESOURCE_NOT_FOUND)
    }

    pub fn try_fetch<T>(&self, id: usize) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>(), id).map(|r| {
            Fetch {
                inner: r.borrow(),
                phantom: PhantomData,
            }
        })
    }

    pub fn fetch_mut<T>(&self, id: usize) -> FetchMut<T>
    where
        T: Resource,
    {
        self.try_fetch_mut(id).expect(RESOURCE_NOT_FOUND)
    }

    pub fn try_fetch_mut<T>(&self, id: usize) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>(), id).map(|r| {
            FetchMut {
                inner: r.borrow_mut(),
                phantom: PhantomData,
            }
        })
    }

    fn try_fetch_internal(&self, id: TypeId, cid: usize) -> Option<&RefCell<Box<Resource>>> {
        self.resources.get(&ResourceId(id, cid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Res;

    #[test]
    fn res_id() {
        assert_eq!(ResourceId::new::<Res>(), ResourceId::new_with_id::<Res>(0));
        assert_eq!(
            ResourceId::new_with_id::<Res>(5),
            ResourceId(TypeId::of::<Res>(), 5)
        );
    }

    #[test]
    fn add() {
        let mut res = Resources::new();
        res.add(Res);

        assert!(res.has_value(ResourceId::new::<Res>()));
        assert!(!res.has_value(ResourceId::new_with_id::<Res>(1)));
        assert!(!res.has_value(ResourceId::new_with_id::<Res>(1)));
    }

    #[test]
    fn fetch_uses_id() {
        let mut res = Resources::new();
        res.add_with_id(5i32, 1);
        res.add_with_id(50i32, 2);

        assert_eq!(*res.fetch::<i32>(1), 5);
        assert_eq!(*res.fetch::<i32>(2), 50);

        {
            *res.fetch_mut::<i32>(1) *= 2;
            *res.fetch_mut::<i32>(2) *= 2;
        }

        {
            assert_eq!(*res.fetch::<i32>(1), 10);
            assert_eq!(*res.fetch::<i32>(2), 100);
        }
    }

    #[test]
    fn mutate() {
        let mut res = Resources::new();
        res.add_with_id(5i32, 1);

        {
            *res.fetch_mut::<i32>(1) *= 2;
        }

        {
            assert_eq!(*res.fetch::<i32>(1), 10);
        }
    }
}
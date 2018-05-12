use bit_set::BitSet;
use component::{Component, ComponentManager};
use std::marker::PhantomData;

pub(crate) trait Aspect {
    fn req(_manager: &ComponentManager) -> BitSet {
        BitSet::new()
    }

    fn not(_manager: &ComponentManager) -> BitSet {
        BitSet::new()
    }
}

pub struct Not<T: Component>(PhantomData<T>);

impl<T> Aspect for T
where
    T: Component,
{
    fn req(manager: &ComponentManager) -> BitSet {
        let mut keys = BitSet::new();
        keys.insert(manager.id::<T>());
        keys
    }
}

impl<T> Aspect for Not<T>
where
    T: Component,
{
    fn not(manager: &ComponentManager) -> BitSet {
        let mut keys = BitSet::new();
        keys.insert(manager.id::<T>());
        keys
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Matcher {
    req: BitSet,
    not: BitSet,
}

impl Matcher {
    pub fn new<T: Aspect>(manager: &ComponentManager) -> Self {
        Matcher {
            req: <T>::req(manager),
            not: <T>::not(manager),
        }
    }

    pub fn check(&self, manager: &ComponentManager, bits: &BitSet) -> bool {
        self.req.intersection(&bits).count() == self.req.len()
            && self.not.intersection(&bits).count() == 0
    }
}

macro_rules! impl_aspect {
    ( $($ty:ident),* ) => {
        impl<$($ty),*> Aspect for ( $( $ty , )* )
            where $( $ty : Aspect ),*
        {
            fn req(manager: &ComponentManager) -> BitSet {
                #![allow(unused_variables, non_snake_case)]

                let mut base = BitSet::new();
                $( base.union_with(&<$ty as Aspect>::req(manager)); )*
                base
            }

            fn not(manager: &ComponentManager) -> BitSet {
                #![allow(unused_variables, non_snake_case)]

                let mut base = BitSet::new();
                $( base.union_with(&<$ty as Aspect>::not(manager)); )*
                base
            }
        }
    };
}

mod impl_aspect {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use super::*;

    impl_aspect!(A);
    impl_aspect!(A, B);
    impl_aspect!(A, B, C);
    impl_aspect!(A, B, C, D);
    impl_aspect!(A, B, C, D, E);
    impl_aspect!(A, B, C, D, E, F);
    impl_aspect!(A, B, C, D, E, F, G);
    impl_aspect!(A, B, C, D, E, F, G, H);
    impl_aspect!(A, B, C, D, E, F, G, H, I);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
    impl_aspect!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
}

#[cfg(test)]
mod tests {
    use super::super::storage::VecStorage;
    use super::*;

    #[derive(Default)]
    struct MyComponent;
    impl Component for MyComponent {
        type Storage = VecStorage<Self>;
    }

    #[derive(Default)]
    struct AnotherComponent;
    impl Component for AnotherComponent {
        type Storage = VecStorage<Self>;
    }

    #[test]
    fn req() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<AnotherComponent>();

        let a1 = <MyComponent>::req(&manager);
        let a2 = <Not<AnotherComponent>>::req(&manager);
        let mut expected = BitSet::new();
        expected.union_with(&a1);
        expected.union_with(&a2);

        assert_eq!(
            expected,
            <(MyComponent, Not<AnotherComponent>)>::req(&manager)
        );
        assert_ne!(
            expected,
            <(MyComponent, Not<AnotherComponent>)>::not(&manager)
        );
    }

    #[test]
    fn not() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<AnotherComponent>();

        let a1 = <MyComponent>::not(&manager);
        let a2 = <Not<AnotherComponent>>::not(&manager);
        let mut expected = BitSet::new();
        expected.union_with(&a1);
        expected.union_with(&a2);

        assert_eq!(
            expected,
            <(MyComponent, Not<AnotherComponent>)>::not(&manager)
        );
        assert_ne!(
            expected,
            <(MyComponent, Not<AnotherComponent>)>::req(&manager)
        );
    }

    #[test]
    fn check() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<AnotherComponent>();

        let mut bits = BitSet::new();
        bits.insert(manager.id::<MyComponent>());
        let matcher = Matcher::new::<(MyComponent, Not<AnotherComponent>)>(&manager);
        assert!(matcher.check(&manager, &bits));
    }

    #[test]
    fn check_has_excluded() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<AnotherComponent>();

        let mut bits = BitSet::new();
        bits.insert(manager.id::<MyComponent>());
        bits.insert(manager.id::<AnotherComponent>());
        let matcher = Matcher::new::<(MyComponent, Not<AnotherComponent>)>(&manager);
        assert!(!matcher.check(&manager, &bits));
    }

    #[test]
    fn check_no_required() {
        let mut manager = ComponentManager::new();
        manager.register::<MyComponent>();
        manager.register::<AnotherComponent>();

        let mut bits = BitSet::new();
        let matcher = Matcher::new::<(MyComponent, Not<AnotherComponent>)>(&manager);
        assert!(!matcher.check(&manager, &bits));
    }
}

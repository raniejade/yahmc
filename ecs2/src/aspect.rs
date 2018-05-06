use std::marker::PhantomData;
use bit_set::BitSet;
use super::component::{Component, ComponentManager};

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
    T: Component
{
    fn req(manager: &ComponentManager) -> BitSet {
        let mut keys = BitSet::new();
        keys.insert(manager.id::<T>());
        keys
    }
}

impl<T> Aspect for Not<T>
where
    T: Component
{
    fn not(manager: &ComponentManager) -> BitSet {
        let mut keys = BitSet::new();
        keys.insert(manager.id::<T>());
        keys
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
    use super::*;
    use super::*;
    struct MyComponent;
    impl Component for MyComponent {}

    struct AnotherComponent;
    impl Component for AnotherComponent {}

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

        assert_eq!(expected, <(MyComponent, Not<AnotherComponent>)>::req(&manager));
        assert_ne!(expected, <(MyComponent, Not<AnotherComponent>)>::not(&manager));
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

        assert_eq!(expected, <(MyComponent, Not<AnotherComponent>)>::not(&manager));
        assert_ne!(expected, <(MyComponent, Not<AnotherComponent>)>::req(&manager));
    }
}
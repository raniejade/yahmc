use bit_set::{BitSet};
use bit_vec::{IntoIter};

trait Join {
    type Item;

    fn open(&self) -> BitSet;

    fn join(self) -> JoinIterator<Self>
    where
        Self: Sized
    {
        JoinIterator::new(self)
    }

    fn get(&self, index: u32) -> Self::Item;
}

struct JoinIterator<T: Join> {
    keys: IntoIter<u32>,
    join: T
}

impl<T> JoinIterator<T>
where
    T: Join
{
    pub fn new(join: T) -> Self {
        let keys = join.open();
        JoinIterator {
            keys: keys.into_bit_vec().into_iter(),
            join
        }
    }
}

impl<T> Iterator for JoinIterator<T>
where
    T: Join
{
    type Item = T::Item;

    fn next(&mut self) -> Option<T::Item> {
        self.keys.next()
            .map(|idx| { self.join.get(idx as u32) })
    }
}

macro_rules! impl_data {
    ( $($ty:ident),* ) => {
        impl<$($ty),*> Join for ( $( $ty , )* )
            where $( $ty : Join ),*
        {
            type Item = ( $($ty::Item,)* );
            fn open(&self) -> BitSet {
                let mut base = BitSet::new();
                let ( $($ty, )* ) = self;
                $( base.intersect_with(&$ty.open()); )*
                base
            }

            fn get(&self, index: u32) -> Self::Item {
                let ( $($ty,)* ) = self;
                ( $( $ty.get(index), )* )
            }
        }
    };
}

mod impl_data {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use super::*;

    impl_data!(A);
    impl_data!(A, B);
    impl_data!(A, B, C);
    impl_data!(A, B, C, D);
    impl_data!(A, B, C, D, E);
    impl_data!(A, B, C, D, E, F);
    impl_data!(A, B, C, D, E, F, G);
    impl_data!(A, B, C, D, E, F, G, H);
    impl_data!(A, B, C, D, E, F, G, H, I);
    impl_data!(A, B, C, D, E, F, G, H, I, J);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
}
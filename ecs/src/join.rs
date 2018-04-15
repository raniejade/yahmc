// pub trait Storage<T> {
//     fn mask(&self) -> BitSet;
//     fn get(&self, index: usize) -> &T;
// }

// pub trait Join { 
//     type Item;
//     type Storage: Storage<Item=Self::Item>;

//     fn open(self) -> (BitSet, Self::Storage);

//     fn join(self) -> JoinIterator<Self>
//     where Self: Sized {
//         JoinIterator::new(self)
//     }
// }

// impl<T, K> Join for K
// where K: Storage<Item=T> {
//     type Item = T;
//     type Storage = Self;
//     fn open(self) -> (BitSet, Self) {
//         (self.mask(), self)
//     }
// }

// pub struct JoinIterator<J: Join> {
//     ids: BitSet,
//     storage: J::Storage
// }

// impl<J: Join> JoinIterator<J> {
//     pub fn new(j: J) -> Self {
//         let (ids, storage) = j.open();
//         JoinIterator {
//             ids,
//             storage
//         }
//     }
// }

// impl<J: Join> Iterator for JoinIterator<J> {
//     type Item = J::Item;

//     fn next(&mut self) -> Option<J::Item> {
//         self.ids.iter()
//             .next()
//             .map(|idx| { self.storage.get(idx) })
//     }
// }
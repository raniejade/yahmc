pub mod storage;

use fxhash::FxHashMap;
use std::any::Any;
use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::default::Default;

use self::storage::{RawStorage, MaskedStorage};
use super::view::{ReadView, WriteView};

pub trait Component: Any + Sized {
    type Storage: RawStorage<Self> + Any + Send + Sync;
}

pub type ReadStorage<'a, T> = ReadView<'a, MaskedStorage<T>>;
pub type WriteStorage<'a, T> = WriteView<'a, MaskedStorage<T>>;
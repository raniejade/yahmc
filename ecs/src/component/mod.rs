pub mod storage;

use fxhash::FxHashMap;
use std::any::Any;
use std::any::TypeId;
use std::default::Default;
use std::cell::{Cell, RefCell};

use self::storage::RawStorage;

pub trait Component: Any + Sized {
    type Storage: RawStorage<Self> +  Any + Send + Sync;
}
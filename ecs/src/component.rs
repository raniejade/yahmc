use fxhash::FxHashMap;
use mopa::Any;
use std::any::TypeId;
use std::default::Default;
use std::cell::{Cell, RefCell};

use super::storage::RawStorage;

pub(crate) type ComponentId = u32;

pub trait Component: Any + Sized {
    type Storage: RawStorage<Self> +  Any + Send + Sync;
}
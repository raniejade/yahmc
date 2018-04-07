use bit_set::BitSet;
use fxhash::FxHashMap;

use super::component::{Component, ComponentId};

pub struct Entity {
    mask: BitSet,
    components: FxHashMap<ComponentId, Box<Component>>
}
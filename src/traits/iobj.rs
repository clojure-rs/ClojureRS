use crate::persistent_list_map::PersistentListMap;
use crate::traits::imeta::IMeta;
use std::fmt::Debug;

// @TODO start swapping PersistentListMap signatures for protocol::IPersistentMap or
// with_meta<I: traits::IPersistentMap>(meta: I)

pub trait IObj: IMeta + Debug {
    fn with_meta(&self, meta: PersistentListMap) -> Self;
}

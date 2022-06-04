use crate::persistent_list_map::PersistentListMap;
use std::fmt::Debug;

pub trait IMeta: Debug {
    fn meta(&self) -> PersistentListMap;
}

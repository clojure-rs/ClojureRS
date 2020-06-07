use std::fmt::Debug;
use crate::persistent_list_map::PersistentListMap;

pub trait IMeta: Debug {
    fn meta(&self) -> PersistentListMap;
}

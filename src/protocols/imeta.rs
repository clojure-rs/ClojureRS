use crate::keyword::Keyword;
use crate::maps::MapEntry;
use crate::persistent_list::PersistentListIter;
use crate::persistent_list::ToPersistentListIter;
use crate::persistent_list_map::ToPersistentListMapIter;
use crate::persistent_list_map::{PersistentListMap, PersistentListMapIter};
use crate::persistent_vector::PersistentVectorIter;
use crate::persistent_vector::ToPersistentVector;
use crate::persistent_vector::ToPersistentVectorIter;
use crate::protocol::Protocol;
use crate::define_protocol;
use crate::symbol::Symbol;
use crate::value::ToValue;
use crate::value::Value;
use std::rc::Rc;

define_protocol!(
    IMeta = PersistentList     |
            PersistentVector   |
            PersistentList     |
            PersistentListMap  |
            Symbol             |
            IFn
);

/// Syntactic sugar, DRY
/// map_entry!("doc", "this is a docstring");
#[macro_export]
macro_rules! map_entry {
    ($key:expr, $value:expr) => {{
        MapEntry {
            key: Keyword::intern($key).to_rc_value(),
            val: $value.to_rc_value(),
        }
    }};
}

/// Syntactic sugar, DRY
/// persistent_list_map!(map_entry!("key1", "value1"), map_entry!("key2", "value2"));
#[macro_export]
macro_rules! persistent_list_map {}
    ( $($kv:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($kv);
            )*
            temp_vec.into_iter().collect::<PersistentListMap>()
        }
    };
}

/// Syntactic sugar, DRY
/// merge!(base_meta(name, ns), map_entry!("key1", "value1"), map_entry!("key2", "value2"));
#[macro_export]
macro_rules! merge {
    ( $plistmap:expr, $($kv:expr), *) => {
        {
            let mut temp_plistmap_as_vec = $plistmap.clone().iter().collect::<Vec<MapEntry>>();
            $(
                temp_plistmap_as_vec.push($kv);
            )*
            temp_plistmap_as_vec.into_iter().collect::<PersistentListMap>()
        }
    };
}

/// Constructs base meta if none provided
/// {:line 1
///  :column 1
///  :file "NO_SOURCE_PATH"
///  :name <something>
///  :ns <namespace>}
///
pub fn base_meta(ns: &str, name: &str) -> PersistentListMap {
    persistent_list_map!(
        map_entry!("line", 1_i32),
        map_entry!("column", 1_i32),
        map_entry!("file", "NO_SOURCE_PATH"),
        map_entry!("ns", Symbol::intern(ns)),
        map_entry!("name", Symbol::intern(name))
    )
}

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
use crate::symbol::Symbol;
use crate::value::ToValue;
use crate::value::Value;
use std::rc::Rc;

//
// This Protocol lives inside of Clojure RS
//
#[derive(Debug, Clone)]
pub struct Meta {
    value: Rc<Value>,
}
impl Protocol for Meta {
    fn try_as_protocol(val: &Rc<Value>) -> Option<Self> {
        match &**val {
            Value::PersistentList(_) => Some(Meta {
                value: Rc::clone(val),
            }),
            Value::PersistentVector(_) => Some(Meta {
                value: Rc::clone(val),
            }),
            Value::PersistentListMap(_) => Some(Meta {
                value: Rc::clone(val),
            }),
            Value::Symbol(_) => Some(Meta {
                value: Rc::clone(val),
            }),
            Value::IFn(_) => Some(Meta {
                value: Rc::clone(val),
            }),
            _ => None,
        }
    }
    fn try_unwrap(&self) -> Option<Rc<Value>> {
        match &*self.value {
            Value::PersistentList(_) => Some(Rc::clone(&self.value)),
            Value::PersistentVector(_) => Some(Rc::clone(&self.value)),
            Value::PersistentListMap(_) => Some(Rc::clone(&self.value)),
            Value::Symbol(_) => Some(Rc::clone(&self.value)),
            Value::IFn(_) => Some(Rc::clone(&self.value)),
            _ => None,
        }
    }
}

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
macro_rules! persistent_list_map {
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
        map_entry!("ns", Symbol::intern_with_ns_empty_meta("", ns)),
        map_entry!("name", Symbol::intern_with_ns_empty_meta("", name))
    )
}

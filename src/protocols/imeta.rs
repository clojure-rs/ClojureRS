use crate::define_protocol;
use crate::keyword::Keyword;
use crate::maps::MapEntry;
use crate::persistent_list_map::PersistentListMap;
use crate::symbol::Symbol;
use crate::traits;
use crate::value::ToValue;
use crate::value::Value;
use std::rc::Rc;

define_protocol!(
    IMeta = Var                | // <-- where all the magic happens
            PersistentList     |
            PersistentVector   |
            PersistentListMap  |
            Symbol //             |
                   // IFn
);
impl traits::IMeta for IMeta {
    fn meta(&self) -> PersistentListMap {
        match &*self.value {
            Value::PersistentList(val) => val.meta(),
            Value::PersistentVector(val) => val.meta(),
            Value::PersistentListMap(val) => val.meta(),
            Value::Symbol(val) => val.meta(),
            Value::Var(var) => var.meta(),
            _ => panic!(
                "protocols::IMeta was wrapping an invalid type {} when calling meta()",
                self.value.type_tag()
            ), // Value::IFn(val) => {
               //     val.with_meta(meta)
               // }
        }
    }
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

mod tests {}

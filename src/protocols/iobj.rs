use crate::define_protocol;
use crate::persistent_list_map::PersistentListMap;
use crate::protocol::ProtocolCastable;
use crate::traits;
use crate::value::{ToValue, Value};
use std::rc::Rc;
// TODO allow nullable protocols
define_protocol!(
    IObj = PersistentList | PersistentVector | PersistentListMap | Symbol //              |
                                                                          // IFn
);
impl traits::IMeta for IObj {
    fn meta(&self) -> PersistentListMap {
        match &*self.value {
            Value::PersistentList(val) => val.meta(),
            Value::PersistentVector(val) => val.meta(),
            Value::PersistentListMap(val) => val.meta(),
            Value::Symbol(val) => val.meta(),
            _ => {
                panic!(
                    "protocols::IMeta was wrapping an invalid type {} when calling meta()",
                    self.value.type_tag()
                )
                //PersistentListMap::Empty
            } // Value::IFn(val) => {
              //     val.with_meta(meta)
              // }
        }
    }
}
impl traits::IObj for IObj {
    fn with_meta(&self, meta: PersistentListMap) -> IObj {
        match &*self.value {
            Value::PersistentList(val) => val.with_meta(meta).to_rc_value().as_protocol::<IObj>(),
            Value::PersistentVector(val) => val.with_meta(meta).to_rc_value().as_protocol::<IObj>(),
            Value::PersistentListMap(val) => {
                val.with_meta(meta).to_rc_value().as_protocol::<IObj>()
            }
            Value::Symbol(val) => val.with_meta(meta).to_rc_value().as_protocol::<IObj>(),
            _ => {
                panic!(
                    "protocols::IMeta was wrapping an invalid type {} when calling meta()",
                    self.value.type_tag()
                )
            } // Value::IFn(val) => {
              //     val.with_meta(meta)
              // }
        }
    }
}
mod tests {}

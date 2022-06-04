use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::persistent_list_map::PersistentListMap;
use crate::protocol::Protocol;
use crate::protocol::ProtocolCastable;
use crate::protocols;
use crate::traits::IObj;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (with-meta obj m)
/// returns object with given metadata
#[derive(Debug, Clone)]
pub struct WithMetaFn {
    enclosing_environment: Rc<Environment>,
}
impl WithMetaFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> WithMetaFn {
        WithMetaFn {
            enclosing_environment,
        }
    }
}
impl ToValue for WithMetaFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for WithMetaFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        }

        match args.get(0).unwrap().try_as_protocol::<protocols::IObj>() {
            Some(obj) => match args.get(1).unwrap().to_value() {
                Value::PersistentListMap(plistmap) => {
                    obj.with_meta(plistmap).unwrap().to_value()
                }
                _ => error_message::type_mismatch(
                    TypeTag::PersistentListMap,
                    args.get(0).unwrap(),
                ),
            }
            // Again, this will likely be swapped for a proper error function, we are currently
            // experimenting with new error messages
            _ => error_message::custom(&format!(
                "In with-meta: first argument is supposed to be of instance IObj, but its type {} is not",
                args.get(0).unwrap().type_tag()
            ))
        }
    }
}

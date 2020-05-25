use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::persistent_list_map::IPersistentMap;
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

        match args.get(0).unwrap().to_value() {
            Value::Symbol(s) => match self.enclosing_environment.get_symbol(&s).to_value() {
                Value::Condition(error) => error_message::unknown_err(error), // TODO should return given value with meta
                // TODO should return value with metadata
                val => match args.get(1).unwrap().to_value() {
                    Value::PersistentListMap(plistmap) => {
                        Value::Symbol(s.with_meta(plistmap.clone()))
                    }
                    _ => error_message::type_mismatch(
                        TypeTag::PersistentListMap,
                        args.get(0).unwrap(),
                    ),
                },
            },
            _ => {
                // TODO : symbol is not the typetag, should be IObj
                return error_message::type_mismatch(TypeTag::Symbol, args.get(0).unwrap());
            }
        }
    }
}

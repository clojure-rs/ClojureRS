use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::type_tag::{type_tag_for_name, TypeTag};
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// deftype-rs, to define a class in clojurers
/// For providing interoperability currently
/// example (deftype-rs "java.lang.String") returns a Value::Class(String) (does not intern)
/// TODO: definitely NOT the same as clojure.core/deftype
#[derive(Debug, Clone)]
pub struct DeftypeRsFn {}
impl ToValue for DeftypeRsFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for DeftypeRsFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        match args.get(0).unwrap().to_value() {
            Value::String(s) => return Value::Class(type_tag_for_name(&s)),
            _ => Value::Condition("not found".to_string()),
        }
    }
}
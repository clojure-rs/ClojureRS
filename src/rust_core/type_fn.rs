use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns type for x
/// example (type x)
#[derive(Debug, Clone)]
pub struct TypeFn {}
impl ToValue for TypeFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for TypeFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        match args.get(0).unwrap().to_value() {
            _any => {
                return Value::Class(_any.type_tag());
            }
        }
    }
}

use crate::error_message;
use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use std::env;
use crate::type_tag::TypeTag;

/// provides a function to return env variables
/// similair to Clojure (System/getenv [key])
#[derive(Debug, Clone)]
pub struct GetEnvFn {}
impl ToValue for GetEnvFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}

impl IFn for GetEnvFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() == 1 {
            match args.get(0).unwrap().to_value() {
                Value::String(key) => {
                    match env::var(key) {
                        Ok(val) => Value::String(val),
                        Err(_) => Value::Nil
                    }
                }
                _a => error_message::type_mismatch(TypeTag::String, &_a)
            }
        } else {
            return error_message::wrong_arg_count(1, args.len());
        }
    }
}

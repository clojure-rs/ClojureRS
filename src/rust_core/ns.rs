use crate::environment::Environment;
use crate::ifn::IFn;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;

#[derive(Debug, Clone)]
pub struct NsMacro {
    enclosing_environment: Rc<Environment>,
}
impl NsMacro {
    pub fn new(enclosing_environment: Rc<Environment>) -> NsMacro {
        NsMacro {
            enclosing_environment,
        }
    }
}
impl ToValue for NsMacro {
    fn to_value(&self) -> Value {
        Value::Macro(Rc::new(self.clone()))
    }
}
impl IFn for NsMacro {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        let namespace = args.get(0).unwrap();
        match &**namespace {
            Value::Symbol(sym) => {
                self.enclosing_environment.change_or_create_namespace(sym);
                Value::Nil
            }
            _ => error_message::type_mismatch(TypeTag::Symbol, &**namespace),
        }
    }
}

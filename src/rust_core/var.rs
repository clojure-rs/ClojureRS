use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::persistent_list_map::IPersistentMap;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns var for symbol
/// example (var println)
#[derive(Debug, Clone)]
pub struct VarFn {
    enclosing_environment: Rc<Environment>,
}
impl VarFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> VarFn {
        VarFn {
            enclosing_environment,
        }
    }
}
impl ToValue for VarFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for VarFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        match args.get(0).unwrap().to_value() {
            Value::Symbol(s) => match self.enclosing_environment.get(&s).to_value() {
                Value::Condition(error) => error_message::unknown_err(error),
                _ => return Value::Var(s),
            },
            _ => {
                return error_message::type_mismatch(TypeTag::Symbol, args.get(0).unwrap());
            }
        }
    }
}

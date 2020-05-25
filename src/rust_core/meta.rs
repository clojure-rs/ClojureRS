use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns meta for symbol
/// Todo: currently uses form (meta 'clojure.string/join)
/// should use #'var-form
/// TODO: macro: true/false
/// TODO: argslists for functions
#[derive(Debug, Clone)]
pub struct MetaFn {
    enclosing_environment: Rc<Environment>,
}
impl MetaFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> MetaFn {
        MetaFn {
            enclosing_environment,
        }
    }
}
impl ToValue for MetaFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for MetaFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        match args.get(0).unwrap().to_value() {
            Value::Var(s) => match self.enclosing_environment.get_symbol(&s).to_value() {
                Value::Condition(error) => error_message::unknown_err(error),
                Value::Symbol(found) => Value::PersistentListMap(found.meta.clone()),
                _s => error_message::unknown_err("target is not IMeta".to_string()),
            },
            _ => Value::Nil,
        }
    }
}

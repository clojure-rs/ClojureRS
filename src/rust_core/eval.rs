use crate::ifn::IFn;
use crate::value::{Value, ToValue, Evaluable};
use std::rc::Rc;
use crate::environment::Environment;

use crate::error_message;

/// (eval form)
///
#[derive(Debug, Clone)]
pub struct EvalFn {
    enclosing_environment: Rc<Environment>,
}
impl EvalFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> EvalFn {
        EvalFn {
            enclosing_environment,
        }
    }
}

impl ToValue for EvalFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for EvalFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len())
        }
        let arg = args.get(0).unwrap();
        arg.eval(Rc::clone(&self.enclosing_environment))
    }
}
use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;

/// Prints system newline, `\n` in rust on all platforms to stdout
/// TODO: should be aware of *out*
/// (defn print-string [string] .. prints single string without linebreak.. )
#[derive(Debug, Clone)]
pub struct SystemNewlineFn {}
impl ToValue for SystemNewlineFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for SystemNewlineFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 0 {
            return error_message::wrong_arg_count(0, args.len());
        }
        print!("\n");
        Value::Nil
    }
}

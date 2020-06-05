use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use std::io;

use crate::error_message;
use std::io::Write;

/// Read a line from stdin TODO: should be aware of *in*
/// (defn read-line [])
#[derive(Debug, Clone)]
pub struct FlushStdoutFn {}
impl ToValue for FlushStdoutFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for FlushStdoutFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 0 {
            return error_message::wrong_arg_count(0, args.len());
        }
        let _ = io::stdout().flush();
        Value::Nil
    }
}

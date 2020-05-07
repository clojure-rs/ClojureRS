use crate::ifn::IFn;
use crate::value::{Value, ToValue, Evaluable};
use std::rc::Rc;

use std::io;

use crate::error_message;
use nom::lib::std::convert::TryFrom;
use std::io::{Read, Write};

/// Read a line from stdin
/// (defn read-line [])
#[derive(Debug, Clone)]
pub struct ReadLineFn {}
impl ToValue for ReadLineFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ReadLineFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 0 {
            return error_message::wrong_arg_count(0, args.len())
        }
        
        let mut input = String::new();
        io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Value::String(input),
            Err(error) => error_message::generic_err(Box::try_from(error).unwrap())
        }
    }
}
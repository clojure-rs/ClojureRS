use crate::value::{ToValue, Value};
use std::rc::Rc;
use crate::ifn::IFn;
use std::io::Read;
use std::error::Error;
use crate::error_message;
use nom::lib::std::convert::TryFrom;
use crate::type_tag::TypeTag;

use std::thread;
use std::time;

/// provides a sleep function to sleep for given amount of ms
#[derive(Debug, Clone)]
pub struct SleepFn {}
impl ToValue for SleepFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}

impl IFn for SleepFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() == 1 {
            let arg = &**args.get(0).unwrap();
            match arg {
                Value::I32(i) => {
                    std::thread::sleep(time::Duration::new(0, (*i as u32) * 1000000));
                    return Value::Nil
                },
                _ => error_message::type_mismatch(TypeTag::I32, args.get(0).unwrap())
            }
        } else {
            error_message::wrong_arg_count(1, args.len());
            return Value::Nil
        }
    }
}
use crate::ifn::IFn;
use crate::value::{Value, ToValue};
use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::error_message;

/// (rand) or (rand n)
///
#[derive(Debug, Clone)]
pub struct RandIntFn {}
impl ToValue for RandIntFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for RandIntFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        match args.len() {
            1 => {
                let arg = args.get(0).unwrap().to_value();
                match arg {
                    Value::I32(i_) => Value::I32(thread_rng().gen_range(0, i_)),
                    Value::F64(f_) => Value::I32(thread_rng().gen_range(0, f_ as i32)),
                    _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                                                   "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                                                   arg.type_tag()
                    ))
                }
            },
            _ => error_message::wrong_arg_count(1, args.len())
        }
    }
}
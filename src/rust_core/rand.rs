use crate::error_message;
use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use rand::{thread_rng, Rng};
use std::rc::Rc;

/// (rand) or (rand n)
///
#[derive(Debug, Clone)]
pub struct RandFn {}
impl ToValue for RandFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for RandFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        match args.len() {
            0 => Value::F64(thread_rng().gen()),
            1 => {
                let arg = args.get(0).unwrap().to_value();
                match arg {
                    Value::I32(i_) => Value::F64(thread_rng().gen_range(0.0, i_ as f64)),
                    Value::F64(f_) => Value::F64(thread_rng().gen_range(0.0, f_)),
                    _ => Value::Condition(format!(
                        // TODO: what error message should be returned regarding using typetags?
                        "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                        arg.type_tag()
                    )),
                }
            }
            _ => error_message::wrong_varg_count(&[0, 1], args.len()),
        }
    }
}

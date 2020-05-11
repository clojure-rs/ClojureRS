use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (str x y & zs)
///
#[derive(Debug, Clone)]
pub struct StrFn {}
impl ToValue for StrFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for StrFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        Value::String(
            args.into_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

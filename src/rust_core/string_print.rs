use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

// Todo: dead code? no usage found
#[derive(Debug, Clone)]
pub struct StringPrintFn {}
impl ToValue for StringPrintFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for StringPrintFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        Value::String(
            args.into_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

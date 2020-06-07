use crate::value::{Value,ToValue};
use std::rc::Rc;
use crate::ifn;

// Let's keep it simple for now, but we will expand this 
define_protocol!(IFn,IFn);

impl ifn::IFn for IFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        match &*self.value {
            Value::IFn(ifn) => {
                ifn.invoke(args)
            },
            _ => panic!("Called Iterable iter on non-iterable"),
        }
    }
}

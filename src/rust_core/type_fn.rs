use crate::error_message;
use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns type for x
/// example (type x)
#[derive(Debug, Clone)]
pub struct TypeFn {}
impl ToValue for TypeFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for TypeFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        match args.get(0).unwrap().to_value() {
            Value::Condition(s) => Value::Condition(s),
            _any => {
                return Value::Class(_any.type_tag());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod type_fn_tests {

        use crate::ifn::IFn;
        use crate::rust_core::TypeFn;
        use crate::type_tag::TypeTag;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn type_fn_test() {
            let type_fn = TypeFn {};
            let s = "whatever";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::Class(TypeTag::String), type_fn.invoke(args));
        }
    }
}

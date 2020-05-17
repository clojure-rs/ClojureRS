use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (assoc map key val & kvs)
///
// General assoc fn; however,  currently just implemented
// for our one map type, PersistentListMap
#[derive(Debug, Clone)]
pub struct EqualsFn {}
impl ToValue for EqualsFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for EqualsFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.is_empty() {
            //@TODO use proper error function
            return Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: > 0)",
                args.len()
            ));
        }

        for pair in args.windows(2) {
            let a = &pair[0];
            let b = &pair[1];
            if a != b {
                return Value::Boolean(false);
            }
        }
        Value::Boolean(true)
    }
}

mod tests {
    use crate::ifn::IFn;
    use crate::keyword::Keyword;
    use crate::rust_core::EqualsFn;
    use crate::value::{ToValue, Value};

    // Just checks that different Values do not count as equal, and that
    // at least one Value of the same kind does, and that one Value of the same
    // kind and different Value doesn't
    //
    // Otherwise, does not test every type
    #[test]
    fn equals_basic() {
        let equals = EqualsFn {};
        let _i32 = Value::I32(1).to_rc_value();
        // To test that we're not getting some sort of 'memory equality'
        let i32_copy = Value::I32(1).to_rc_value();
        assert!(equals
            .invoke(vec![i32_copy.clone(), _i32.clone()])
            .is_truthy());
        assert!(equals.invoke(vec![_i32.clone(), _i32.clone()]).is_truthy());

        let i32_2 = Value::I32(5).to_rc_value();
        assert!(!equals.invoke(vec![_i32.clone(), i32_2.clone()]).is_truthy());

        let keyword = Keyword::intern("cat").to_rc_value();
        let keyword2 = Keyword::intern("cat").to_rc_value();
        let keyword3 = Keyword::intern("dog").to_rc_value();

        assert!(equals
            .invoke(vec![keyword.clone(), keyword2.clone()])
            .is_truthy());
        assert!(!equals.invoke(vec![keyword2, keyword3]).is_truthy());
        assert!(!equals.invoke(vec![keyword, _i32]).is_truthy());
    }
}

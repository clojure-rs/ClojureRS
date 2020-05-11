use crate::ifn::IFn;
use crate::symbol::Symbol;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::persistent_list::ToPersistentList;

/// (do body)
///
#[derive(Debug, Clone)]
pub struct DoFn {}
impl ToValue for DoFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for DoFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.is_empty() {
            return Value::Nil;
        }
        (**args.last().unwrap()).clone()
    }
}

//
// Since our macros currently expand and evaluate at the same time,  our `do` macro will be implemented
// by expanding to a do-fn, which will just naturally evaluate all arguments, being a fn, and
// return the last item
// This will change when macros change
//
#[derive(Debug, Clone)]
pub struct DoMacro {}
impl ToValue for DoMacro {
    fn to_value(&self) -> Value {
        Value::Macro(Rc::new(self.clone()))
    }
}
impl IFn for DoMacro {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.is_empty() {
            return vec![Symbol::intern("do").to_rc_value(), Rc::new(Value::Nil)]
                .into_list()
                .to_value();
        }
        // (do a b c) becomes (do-fn* a b c), so we need to copy a,b, and c for our new expression
        let args_for_ret_expr = args
            .iter()
            .map(|arg| arg.to_rc_value())
            .collect::<Vec<Rc<Value>>>();

        let mut do_body = vec![Symbol::intern("do-fn*").to_rc_value()];
        do_body.extend_from_slice(args_for_ret_expr.get(0..).unwrap());

        do_body.into_list().to_value()
    }
}

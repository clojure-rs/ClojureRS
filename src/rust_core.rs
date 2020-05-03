use crate::value::Value;
use std::rc::Rc;

use crate::environment::Environment;
use crate::ifn::IFn;
use crate::persistent_list::{
    PersistentList,
    PersistentList::{Cons, Empty},
    ToPersistentList, ToPersistentListIter,
};
use crate::persistent_vector::{PersistentVector, ToPersistentVectorIter};
use crate::symbol::Symbol;
use crate::value::{Evaluable, ToValue};

use crate::error_message;
use crate::type_tag::TypeTag;

//
// This module will hold the core functions and macros that Clojure will
// hook into; Functions / Macros like "+", "fn*", "let", "cond", etc
//
// This is still experimental, and we may instead undo this all and
// represent fn* and let and the like the same it is done in ClojureJVM,
// where I believe they are defined flat out in the Compiler class
//
// However, even in that case this may stick around to implement basic
// functions like "+" that usually rely on interop
//

#[derive(Debug, Clone)]
pub struct StrFn {}
impl ToValue for StrFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for StrFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        Value::String(
            args.into_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

#[derive(Debug, Clone)]
pub struct StringPrintFn {}
impl ToValue for StringPrintFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for StringPrintFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        Value::String(
            args.into_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

#[derive(Debug, Clone)]
pub struct AddFn {}
impl ToValue for AddFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for AddFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        args.into_iter().fold(0_i32.to_value(), |a, b| match a {
            Value::I32(a_) => match b {
                Value::I32(b_) => Value::I32(a_ + b_),
                _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                    "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                    b.type_tag()
                )),
            },
            _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                "Type mismatch: Expecting: (i32 | i64 | f32 | f64), Found: {}",
                a.type_tag()
            )),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EvalFn {
    enclosing_environment: Rc<Environment>,
}
impl EvalFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> EvalFn {
        EvalFn {
            enclosing_environment,
        }
    }
}
impl ToValue for EvalFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for EvalFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len())
        }
        let arg = args.get(0).unwrap();
        arg.eval(Rc::clone(&self.enclosing_environment))
    }
}

#[derive(Debug, Clone)]
pub struct DoFn {}
impl ToValue for DoFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for DoFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.is_empty() {
            return Value::Nil;
        }
        (*args.last().unwrap()).clone()
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
    fn invoke(&self, args: Vec<&Value>) -> Value {
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

#[derive(Debug, Clone)]
pub struct NthFn {}
impl ToValue for NthFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for NthFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.len() != 2 {
            return error_message::wrong_varg_count(&[2,3], args.len())
        }
        // @TODO change iteration to work with Value references, or even change invoke to work on Rc<..>
        //       as we do everything else; surely we don't want to clone just to read from a collection
        if let Some(Value::I32(ind)) = args.get(1) {
            if *ind < 0 {
                return error_message::index_cannot_be_negative(*ind as usize)
            }
            let ind = *ind as usize;

            match args.get(0).unwrap() {
                Value::PersistentList(Cons(head, tail, count)) => {
                    let count = *count as usize;
                    if ind >= count {
                        error_message::index_out_of_bounds(ind, count)
                    } else if ind == 0 {
                        head.to_value()
                    } else {
                        tail.iter().nth(ind - 1).unwrap().to_value()
                    }
                }
                Value::PersistentList(Empty) => error_message::index_out_of_bounds(ind, 0),
                Value::PersistentVector(PersistentVector { vals }) => {
                    if ind >= vals.len() {
                        error_message::index_out_of_bounds(ind, vals.len())
                    } else {
                        vals.get(ind).unwrap().to_value()
                    }
                }
                _ => error_message::type_mismatch(TypeTag::ISeq, args.get(0)),
            }
        } else {
            error_message::type_mismatch(TypeTag::Integer, args.get(1))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConcatFn {}
impl ToValue for ConcatFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ConcatFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        let concatted_vec = args.iter().fold(vec![], |mut sum, coll| {
            let coll_vec = match coll {
                Value::PersistentList(plist) => {
                    Rc::new(plist.clone()).iter().collect::<Vec<Rc<Value>>>()
                }
                Value::PersistentVector(pvector) => {
                    Rc::new(pvector.clone()).iter().collect::<Vec<Rc<Value>>>()
                }
                _ => vec![],
            };

            sum.extend(coll_vec);
            sum
        });
        Value::PersistentList(concatted_vec.into_iter().collect::<PersistentList>())
    }
}

/// Primitive printing function;
/// (defn print-string [string] .. prints single string .. )
#[derive(Debug, Clone)]
pub struct PrintStringFn {}
impl ToValue for PrintStringFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for PrintStringFn {
    fn invoke(&self, args: Vec<&Value>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len())
        }
        println!("{}", args.get(0).unwrap().to_string());
        Value::Nil
    }
}

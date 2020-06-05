use crate::environment::Environment;
use crate::ifn::IFn;
use crate::value::{Evaluable, ToValue, Value};
use std::rc::Rc;

use crate::iterable::Iterable;
use crate::persistent_list::{PersistentList, ToPersistentList};
use crate::persistent_vector::PersistentVector;
use crate::protocol::ProtocolCastable;
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::{error_message, lambda};

/// Special Form implementation
/// (fn* [& fdecl])
/// i.e. (fn* [a] (println "hello " a))
/// supports also other possible forms met in clojure.core, but does nothing
/// https://stackoverflow.com/questions/7552632/what-does-static-do-in-clojure
/// possible deprecation, needs to be inspected
/// (fn* let [a b c] ...)
/// (fn* ^Static let [ ] ...)
pub fn FnStar(environment: &Rc<Environment>, args: &Rc<PersistentList>) -> Option<Rc<Value>> {
    let arg_rc_values = PersistentList::iter(args)
        .map(|rc_arg| rc_arg)
        .collect::<Vec<Rc<Value>>>();

    if arg_rc_values.is_empty() {
        return Some(Rc::new(Value::Condition(format!(
            "Wrong number of arguments (Given: {}, Expect: >=1",
            arg_rc_values.len()
        ))));
    }

    // skip optionally deprecated forms
    let mut arg_index = 0;
    arg_index = match arg_rc_values.get(0).unwrap().to_value() {
        Value::Symbol(_) => arg_index + 1,
        _ => arg_index,
    };
    if args.len() > 1 {
        arg_index = match arg_rc_values.get(1).unwrap().to_value() {
            Value::Symbol(_) => arg_index + 1,
            _ => arg_index,
        };
    }

    if arg_index > args.len() - 1 {
        return Some(Rc::new(Value::Condition(std::string::String::from(
            "help: (fn* meta-kw? symbol-name? argv body)",
        ))));
    }

    let fn_args = arg_rc_values.get(arg_index as usize).unwrap();

    match &**fn_args {
        Value::PersistentVector(PersistentVector { vals }) => {
            let mut arg_syms_vec = vec![];
            let enclosing_environment =
                Rc::new(Environment::new_local_environment(Rc::clone(&environment)));
            for val in vals.iter() {
                if let Value::Symbol(sym) = &**val {
                    arg_syms_vec.push(sym.clone());
                }
            }

            let fn_body =
                // (fn [x y] ) -> nil 
                if (arg_rc_values.len()- arg_index as usize) <= 1 {
                    Rc::new(Value::Nil)
                    // (fn [x y] expr) -> expr 
                } else if arg_rc_values.len() == 2 {
                    Rc::clone(arg_rc_values.get((arg_index  + 1) as usize).unwrap())
                    // (fn [x y] expr1 expr2 expr3) -> (do expr1 expr2 expr3) 
                } else {
                    // (&[expr1 expr2 expr3] 
                    let body_exprs = arg_rc_values.get(1..).unwrap();
                    // vec![do]
                    let mut do_body = vec![Symbol::intern("do").to_rc_value()];
                    // vec![do expr1 expr2 expr3]
                    do_body.extend_from_slice(body_exprs);
                    // (do expr1 expr2 expr3) 
                    do_body.into_list().to_rc_value()
                };

            Some(Rc::new(
                lambda::Fn {
                    body: fn_body,
                    enclosing_environment,
                    arg_syms: arg_syms_vec,
                }
                .to_value(),
            ))
        }
        _ => Some(Rc::new(Value::Condition(std::string::String::from(
            "help: (fn* meta-kw? symbol-name? argv body)",
        )))),
    }
}

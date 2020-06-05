use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::keyword::Keyword;
use crate::persistent_vector::ToPersistentVectorIter;
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::util::IsOdd;
use crate::value::{ToValue, Value};
use if_chain::if_chain;
use itertools::Itertools;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ReferFn {
    enclosing_environment: Rc<Environment>,
}
impl ReferFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> ReferFn {
        ReferFn {
            enclosing_environment,
        }
    }
}
impl ToValue for ReferFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ReferFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // (refer ns) , (refer ns :filter ..) , (refer ns :filter1 .. :filter2 ..) ..
        if !args.len().is_odd() {
            return error_message::wrong_arg_count(1, args.len());
        }

        let namespace = args.get(0).unwrap();
        if args.len() > 1 {
            for (filter_key, filter_val) in args.iter().skip(1).tuples() {
                // To avoid ungodly nesting
                // Ex: 'ns :only [a b c d]
                if_chain! {
                    if let Value::Symbol(ns) = &**namespace;
                    if let Value::Keyword(Keyword{sym: Symbol{name: filter_name,..}}) = &**filter_key;
                    if filter_name == "only";
                    if let Value::PersistentVector(pvector) = &**filter_val;
                    then {
                        // @TODO definitely need to rename this elsewhere in codebase, where we've just been calling it 'syms'
                        let mut referred_syms_map = HashMap::new();
                        // We're going to get our vector of symbols as a pvector of Rc<Value>, we need to convert that
                        // into a vector of symbols
                        let mut sym_vector = vec![];
                        let rc_pvector = Rc::new(pvector.clone());
                        for maybe_rc_sym in rc_pvector.iter() {
                            if let Value::Symbol(sym) = &*maybe_rc_sym {
                                sym_vector.push(sym.unqualified());
                            }
                            else {
                                return error_message::type_mismatch(TypeTag::Symbol, &*maybe_rc_sym);
                            }
                        }
                        referred_syms_map.insert(ns.unqualified(),sym_vector);
                        self.enclosing_environment.add_referred_syms_to_curr_namespace(referred_syms_map);
                    }
                }
            }
        } else {
            if let Value::Symbol(ns) = &**namespace {
                self.enclosing_environment
                    .add_referred_namespace_to_curr_namespace(ns);
            }
        }
        Value::Nil
    }
}

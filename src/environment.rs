use crate::namespace::{Namespace, Namespaces};
use crate::repl;
use crate::repl::Repl;
use crate::rust_core;
use crate::symbol::Symbol;
use crate::value::{ToValue, Value};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// @TODO lookup naming convention
/// Inner value of our environment
/// See Environment for overall purpose
#[derive(Debug, Clone)]
pub struct EnvironmentVal {
    curr_ns: Namespace,
    namespaces: Namespaces,
}
impl EnvironmentVal {
    /// Default main environment
    fn new_main_val() -> EnvironmentVal {
        EnvironmentVal {
            curr_ns: Namespace::new(Symbol::intern("user"), RefCell::new(HashMap::new())),
            namespaces: Namespaces(RefCell::new(HashMap::new())),
        }
    }
}
/// Our environment keeps track of the meaning of things 'right here', relative to where
/// something is at (meaning, a form inside of a let might have a different meaning for
/// the symbol x than a form outside of it, with a let introducing an additional local environment
///
/// Stores our namespaces and our current namespace, which themselves personally store our symbols
/// mapped to values
#[derive(Debug, Clone)]
pub enum Environment {
    MainEnvironment(EnvironmentVal),
    /// Points to parent environment
    /// Introduced by Closures, and by let
    LocalEnvironment(Rc<Environment>, RefCell<HashMap<Symbol, Rc<Value>>>),
}
use Environment::*;
impl Environment {
    pub fn new_main_environment() -> Environment {
        MainEnvironment(EnvironmentVal::new_main_val())
    }
    pub fn new_local_environment(outer_environment: Rc<Environment>) -> Environment {
        LocalEnvironment(outer_environment, RefCell::new(HashMap::new()))
    }
    pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
        match self {
            MainEnvironment(EnvironmentVal { curr_ns, .. }) => {
                curr_ns.insert(sym, val);
            }
            LocalEnvironment(_, mappings) => {
                mappings.borrow_mut().insert(sym, val);
            }
        }
    }
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self {
            MainEnvironment(EnvironmentVal { curr_ns, .. }) => curr_ns.get(sym),

            LocalEnvironment(parent_env, mappings) => match mappings.borrow().get(sym) {
                Some(val) => Rc::clone(val),
                None => parent_env.get(sym),
            },
        }
    }
    pub fn clojure_core_environment() -> Rc<Environment> {
        // Register our macros / functions ahead of time
        let add_fn = rust_core::AddFn {};
        let str_fn = rust_core::StrFn {};
        let do_fn = rust_core::DoFn {};
        let nth_fn = rust_core::NthFn {};
        let do_macro = rust_core::DoMacro {};
        let concat_fn = rust_core::ConcatFn {};
        let print_string_fn = rust_core::PrintStringFn {};
        // Hardcoded fns
        let lexical_eval_fn = Value::LexicalEvalFn {};
        // Hardcoded macros
        let let_macro = Value::LetMacro {};
        let quote_macro = Value::QuoteMacro {};
        let def_macro = Value::DefMacro {};
        let fn_macro = Value::FnMacro {};
        let defmacro_macro = Value::DefmacroMacro {};
        let environment = Rc::new(Environment::new_main_environment());

        let eval_fn = rust_core::EvalFn::new(Rc::clone(&environment));

        environment.insert(Symbol::intern("+"), add_fn.to_rc_value());
        environment.insert(Symbol::intern("let"), let_macro.to_rc_value());
        environment.insert(Symbol::intern("str"), str_fn.to_rc_value());
        environment.insert(Symbol::intern("quote"), quote_macro.to_rc_value());
        environment.insert(Symbol::intern("def"), def_macro.to_rc_value());
        environment.insert(Symbol::intern("fn"), fn_macro.to_rc_value());
        environment.insert(Symbol::intern("defmacro"), defmacro_macro.to_rc_value());
        environment.insert(Symbol::intern("eval"), eval_fn.to_rc_value());

        environment.insert(Symbol::intern("+"), add_fn.to_rc_value());
        environment.insert(Symbol::intern("let"), let_macro.to_rc_value());
        environment.insert(Symbol::intern("str"), str_fn.to_rc_value());
        environment.insert(Symbol::intern("quote"), quote_macro.to_rc_value());
        environment.insert(Symbol::intern("do-fn*"), do_fn.to_rc_value());
        environment.insert(Symbol::intern("do"), do_macro.to_rc_value());
        environment.insert(Symbol::intern("def"), def_macro.to_rc_value());
        environment.insert(Symbol::intern("fn"), fn_macro.to_rc_value());
        environment.insert(Symbol::intern("defmacro"), defmacro_macro.to_rc_value());
        environment.insert(Symbol::intern("eval"), eval_fn.to_rc_value());
        environment.insert(
            Symbol::intern("lexical-eval"),
            lexical_eval_fn.to_rc_value(),
        );

        environment.insert(Symbol::intern("nth"), nth_fn.to_rc_value());
        environment.insert(Symbol::intern("concat"), concat_fn.to_rc_value());
        environment.insert(
            Symbol::intern("print-string"),
            print_string_fn.to_rc_value(),
        );

        //
        // Read in clojure.core
        //
        // @TODO its time for a RT (runtime), which environment seems to be becoming
        let _ = Repl::new(Rc::clone(&environment)).try_eval_file("./src/clojure/core.clj");

        environment
    }
}

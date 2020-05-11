use crate::clojure_std;
use crate::namespace::{Namespace, Namespaces};
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
    //@TODO is it worth just making this a mutable reference (to an
    // immutable value), and referencing the current symbol at any
    // point in time?  Is implementing that sort of speedup in general
    // significant
    curr_ns_sym: RefCell<Symbol>,
    namespaces: Namespaces,
}
impl EnvironmentVal {
    fn change_namespace(&self,name: Symbol){
	self.curr_ns_sym.replace(name);
    }
    fn insert_into_namespace(&self,namespace: &Symbol, sym: Symbol, val: Rc<Value>) {
	self.namespaces.insert_into_namespace(namespace,sym,val);
    }	
    fn insert_into_current_namespace(&self,sym: Symbol, val: Rc<Value>){
	self.namespaces.insert_into_namespace(&*self.curr_ns_sym.borrow(),sym,val);
    }
    fn get_from_namespace(&self,namespace: &Symbol,sym: &Symbol) -> Rc<Value>
    {
	self.namespaces.get(namespace,sym)
    }
    fn get_current_namespace(&self) -> Symbol {
	self.curr_ns_sym.borrow().clone()
    }
    // @TODO as mentioned, we've been working with a memory model where values exist
    //       in our system once-ish and we reference them all over with Rc<..>
    //       Look into possibly working this into that (if its even significant);
    /// Default main environment
    fn new_main_val() -> EnvironmentVal {
	let curr_ns_sym = Symbol::intern("user");
	let curr_ns     = Namespace::from_sym(curr_ns_sym.clone());
	let namespaces  = Namespaces(RefCell::new(HashMap::new()));
	namespaces.insert(curr_ns_sym.clone(),curr_ns);
        EnvironmentVal {
            curr_ns_sym: RefCell::new(curr_ns_sym),
            namespaces
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
    pub fn change_namespace(&self,symbol: Symbol) {
	let symbol = symbol.unqualified();
	
	match self.get_main_environment() {
	    MainEnvironment(EnvironmentVal { curr_ns_sym, ..}) => {
		curr_ns_sym.replace(symbol);
	    },
	    LocalEnvironment(..) => {
		panic!(
		    "get_main_environment() returns LocalEnvironment,\
		     but by definition should only return MainEnvironment"
		)
	    }
	}
    }
    // @TODO consider 'get_current_..' for consistency?
    // @TODO consider 'current_namespace_sym'? after all, its not the namespace itself
    pub fn get_current_namespace(&self) -> Symbol {
	match self.get_main_environment() {
	    MainEnvironment(EnvironmentVal { curr_ns_sym, ..}) =>
		curr_ns_sym.borrow().clone(),
	    LocalEnvironment(..) => {
		panic!(
		    "In get_current_namespace_name(): get_main_environment() returns LocalEnvironment,\
		     but by definition should only return MainEnvironment"
		)
	    }
	}
    }
    // Note; since we're now dealing with curr_ns as a refcell, we're
    // returning a String instead of a &str, as I suspect a &str could
    // risk becoming invalid as curr_ns changes
    pub fn get_current_namespace_name(&self) -> String {
	self.get_current_namespace().name.clone()
    }
    
    pub fn new_main_environment() -> Environment {
        MainEnvironment(EnvironmentVal::new_main_val())
    }
    pub fn new_local_environment(outer_environment: Rc<Environment>) -> Environment {
        LocalEnvironment(outer_environment, RefCell::new(HashMap::new()))
    }
    /// Insert a binding into an arbitrary namespace
    fn insert_into_namespace(&self,namespace: &Symbol, sym: Symbol, val: Rc<Value>) {
	match self.get_main_environment() {
	    MainEnvironment(env_val) => env_val.insert_into_namespace(namespace,sym,val),
	    LocalEnvironment(..) => {
		panic!(
		    "get_main_environment() returns LocalEnvironment,\
		     but by definition should only return MainEnvironment"
		)
	    }
	}
    }
    pub fn insert_into_current_namespace(&self,sym: Symbol, val: Rc<Value>){
	match self.get_main_environment() {
	    MainEnvironment(env_val) => env_val.insert_into_current_namespace(sym,val),
	    LocalEnvironment(..) => {
		panic!(
		    "get_main_environment() returns LocalEnvironment,\
		     but by definition should only return MainEnvironment"
		)
	    }
	}
    }
    /// Insert into the environment around you;  the local bindings,
    /// or the current namespace, if this is top level
    /// For instance,
    /// ```clojure
    ///   (def a 1)      ;; => main_environment.insert(a,1)
    ///   (let [a 1] ..) ;; => local_environment.insert(a,1)  
    pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
        match self {
            MainEnvironment(_) => { self.insert_into_current_namespace(sym, val);
            }
            LocalEnvironment(_, mappings) => {
                mappings.borrow_mut().insert(sym, val);
            }
        }
    }
    fn get_main_environment(&self) -> &Self {
	match self {
            MainEnvironment(_) => self,
            LocalEnvironment(parent_env, ..) => parent_env.get_main_environment() 
        }
    }

    // @TODO figure out convention for 'ns' vs 'namespace'
    /// Get closest value "around" us;  try our local environment, then
    /// try our main environment (unless its namespace qualified)
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self {
            MainEnvironment(env_val) => { 
		// If we've recieved a qualified symbol like
		// clojure.core/+ 
		if sym.ns != "" {
		    // Use that namespace 
		    env_val.get_from_namespace(&Symbol::intern(&sym.ns),sym)
		}
		else {
		    env_val.get_from_namespace(
			&env_val.get_current_namespace(),
			&Symbol::intern(&sym.name)
		    )
		}
	    },
            LocalEnvironment(parent_env, mappings) => {
		if sym.ns != "" {
		    return self.get_main_environment().get(sym);
		}
		match mappings.borrow().get(sym) {
                    Some(val) => Rc::clone(val),
                    None => parent_env.get(sym),
		}
	    }
        }
    }
    pub fn clojure_core_environment() -> Rc<Environment> {
        // Register our macros / functions ahead of time
        let add_fn = rust_core::AddFn {};
        let subtract_fn = rust_core::SubtractFn {};
        let multiply_fn = rust_core::MultiplyFn {};
        let divide_fn = rust_core::DivideFn {};
        let rand_fn = rust_core::RandFn {};
        let rand_int_fn = rust_core::RandIntFn {};
        let str_fn = rust_core::StrFn {};
        let do_fn = rust_core::DoFn {};
        let nth_fn = rust_core::NthFn {};
        let do_macro = rust_core::DoMacro {};
        let concat_fn = rust_core::ConcatFn {};
        let print_string_fn = rust_core::PrintStringFn {};
        let assoc_fn = rust_core::AssocFn {};

        // rust implementations of core functions
        let slurp_fn = rust_core::slurp::SlurpFn {};

        // clojure.std functions
        let thread_sleep_fn = clojure_std::thread::SleepFn {};
        let nanotime_fn = clojure_std::time::NanoTimeFn {};

	      let get_fn = rust_core::GetFn {};
        let map_fn = rust_core::MapFn {};
	
        // Hardcoded fns
        let lexical_eval_fn = Value::LexicalEvalFn {};
        // Hardcoded macros
        let let_macro = Value::LetMacro {};
        let quote_macro = Value::QuoteMacro {};
        let def_macro = Value::DefMacro {};
        let fn_macro = Value::FnMacro {};
        let defmacro_macro = Value::DefmacroMacro {};
	      let if_macro = Value::IfMacro {};
        let environment = Rc::new(Environment::new_main_environment());

	      let eval_fn = rust_core::EvalFn::new(Rc::clone(&environment));
	      let ns_macro = rust_core::NsMacro::new(Rc::clone(&environment));
        let load_file_fn = rust_core::LoadFileFn::new(Rc::clone(&environment));
	      // @TODO after we merge this with all the other commits we have,
	      //       just change all the `insert`s here to use insert_in_namespace
	      //       I prefer explicity and the non-dependence-on-environmental-factors
	      environment.change_namespace(Symbol::intern("clojure.core"));


        environment.insert(Symbol::intern("+"), add_fn.to_rc_value());
        environment.insert(Symbol::intern("-"), subtract_fn.to_rc_value());
        environment.insert(Symbol::intern("*"), multiply_fn.to_rc_value());
        environment.insert(Symbol::intern("_slash_"), divide_fn.to_rc_value());
        environment.insert(Symbol::intern("rand"), rand_fn.to_rc_value());
        environment.insert(Symbol::intern("rand-int"), rand_int_fn.to_rc_value());
        environment.insert(Symbol::intern("let"), let_macro.to_rc_value());
        environment.insert(Symbol::intern("str"), str_fn.to_rc_value());
        environment.insert(Symbol::intern("quote"), quote_macro.to_rc_value());
        environment.insert(Symbol::intern("def"), def_macro.to_rc_value());
        environment.insert(Symbol::intern("fn"), fn_macro.to_rc_value());
        environment.insert(Symbol::intern("defmacro"), defmacro_macro.to_rc_value());
        environment.insert(Symbol::intern("eval"), eval_fn.to_rc_value());

        // Thread namespace TODO / instead of _
        environment.insert(
            Symbol::intern("Thread_sleep"),
            thread_sleep_fn.to_rc_value(),
        );

        environment.insert(
	    Symbol::intern("System_nanotime"),
	    nanotime_fn.to_rc_value()
	);

        // core.clj wraps calls to the rust implementations
        // @TODO add this to clojure.rs.core namespace as clojure.rs.core/slurp
        environment.insert(Symbol::intern("rust-slurp"), slurp_fn.to_rc_value());

        environment.insert(Symbol::intern("+"), add_fn.to_rc_value());
        environment.insert(Symbol::intern("let"), let_macro.to_rc_value());
        environment.insert(Symbol::intern("str"), str_fn.to_rc_value());
        environment.insert(Symbol::intern("map"), map_fn.to_rc_value());

        environment.insert(Symbol::intern("quote"), quote_macro.to_rc_value());
        environment.insert(Symbol::intern("do-fn*"), do_fn.to_rc_value());
        environment.insert(Symbol::intern("do"), do_macro.to_rc_value());
        environment.insert(Symbol::intern("def"), def_macro.to_rc_value());
        environment.insert(Symbol::intern("fn"), fn_macro.to_rc_value());
	      environment.insert(Symbol::intern("if"), if_macro.to_rc_value());
        environment.insert(Symbol::intern("defmacro"), defmacro_macro.to_rc_value());
	environment.insert(Symbol::intern("ns"), ns_macro.to_rc_value());
        environment.insert(Symbol::intern("eval"), eval_fn.to_rc_value());
        environment.insert(
            Symbol::intern("lexical-eval"),
            lexical_eval_fn.to_rc_value(),
        );
	      environment.insert(Symbol::intern("load-file"), load_file_fn.to_rc_value());
        environment.insert(Symbol::intern("nth"), nth_fn.to_rc_value());
	      environment.insert(Symbol::intern("assoc"), assoc_fn.to_rc_value());
	      environment.insert(Symbol::intern("get"), get_fn.to_rc_value());
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

	// We can add this back once we have requires
	// environment.change_namespace(Symbol::intern("user"));

        environment
    }
}


#[cfg(test)]
mod tests {    
    mod environment_val_tests {
	use crate::environment::Environment;
	use crate::environment::Environment::*;
	use crate::environment::EnvironmentVal;
	use crate::symbol::Symbol;
	use crate::value::{ToValue,Value};
	use crate::ifn::IFn;
	use crate::rust_core;
	use std::rc::Rc;

	//////////////////////////////////////////////////////////////////////////////////////////////////////
	//
	// pub fn get_current_namespace(&self) -> Symbol {
	//
	//////////////////////////////////////////////////////////////////////////////////////////////////////
	
	#[test]
        fn test_get_current_namespace() {
	    let env_val = EnvironmentVal::new_main_val();

	    assert_eq!(Symbol::intern("user"),env_val.get_current_namespace());

	    env_val.change_namespace(Symbol::intern("core"));
	    assert_eq!(Symbol::intern("core"),env_val.get_current_namespace());

	    // @TODO add this invariant back next, and remove this comment; 5.9.2020
	    // env_val.change_namespace(Symbol::intern_with_ns("not-ns","ns"));
	    // assert_eq!(Symbol::intern("ns"),env_val.get_current_namespace())
		;
				     
	    // @TODO add case for local environment 	
	}
	
	////////////////////////////////////////////////////////////////////////////////////////////////////
	//
	//  fn get_from_namespace(&self,namespace: &Symbol,sym: &Symbol) -> Rc<Value>
	//
	////////////////////////////////////////////////////////////////////////////////////////////////////
	
	#[test]
        fn test_get_from_namespace() {
	    let env_val = EnvironmentVal::new_main_val();

	    env_val.insert_into_namespace(
		&Symbol::intern("core"),Symbol::intern("+"),Rc::new(Value::Nil)
	    );
	    env_val.insert_into_namespace(
		&Symbol::intern_with_ns("dragon","core"),
		Symbol::intern("+2"),
		Rc::new(Value::Nil)
	    );
	    env_val.insert_into_namespace(
		&Symbol::intern_with_ns("dragon","core"),
		Symbol::intern_with_ns("override","+3"),
		Rc::new(Value::Nil)
	    );

	    assert_eq!(Rc::new(Value::Nil),
		      env_val.get_from_namespace(
			  &Symbol::intern("core"),
			  &Symbol::intern("+")
		      ));

	    assert_eq!(Rc::new(Value::Nil),
		      env_val.get_from_namespace(
			  &Symbol::intern("core"),
			  &Symbol::intern("+2")
		      ));

	    assert_eq!(Rc::new(Value::Nil),
		      env_val.get_from_namespace(
			  &Symbol::intern("override"),
			  &Symbol::intern("+3")
		      ));
	
	}
	////////////////////////////////////////////////////////////////////////////////////////////////////
	//  get_from_namespace
	////////////////////////////////////////////////////////////////////////////////////////////////////
	
    }
    mod environment_tests {
	use crate::environment::Environment;
	use crate::environment::Environment::*;
	use crate::environment::EnvironmentVal;
	use crate::symbol::Symbol;
	use crate::value::{ToValue,Value};
	use crate::ifn::IFn;
	use crate::rust_core;
	use std::rc::Rc;
	////////////////////////////////////////////////////////////////////////////////////////////////////
	//
	// pub fn get(&self, sym: &Symbol) -> Rc<Value> {
	//
	////////////////////////////////////////////////////////////////////////////////////////////////////
	#[test]
        fn test_get__plus() {
	    let add_fn = rust_core::AddFn {};
	    
	    let environment = Rc::new(Environment::new_main_environment());
	    environment.insert(Symbol::intern("+"),add_fn.to_rc_value());

	    let plus = environment.get(&Symbol::intern("+"));

	    assert_eq!(8.to_value(),add_fn.invoke(vec![3_i32.to_rc_value(),5_i32.to_rc_value()]));
	    
	    if let Value::IFn(add_ifn) = &*plus {
		assert_eq!(8.to_value(),add_ifn.invoke(vec![3_i32.to_rc_value(),5_i32.to_rc_value()]));
		return;
	    }
	    panic!("test_get_plus: plus is: {:#?}",plus);
	}
	////////////////////////////////////////////////////////////////////////////////////////////////////
	//
	// pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
	//
	////////////////////////////////////////////////////////////////////////////////////////////////////
	#[test]
        fn test_insert__plus() {
	    let add_fn = rust_core::AddFn {};
	    
	    let environment = Rc::new(Environment::new_main_environment());
	    environment.insert(Symbol::intern("+"),add_fn.to_rc_value());

	    let plus : Rc<Value> = match &*environment {
		MainEnvironment(EnvironmentVal { curr_ns_sym, namespaces }) => {
		    namespaces
			.0
			.borrow()
			.get(&Symbol::intern("user"))
			.unwrap()
			.get(&Symbol::intern("+"))
		},
		_ => panic!("new_main_environment() should return Main")
	    };

	    assert_eq!(8.to_value(),add_fn.invoke(vec![3_i32.to_rc_value(),5_i32.to_rc_value()]));
	    
	    if let Value::IFn(add_ifn) = &*plus {
		assert_eq!(8.to_value(),add_ifn.invoke(vec![3_i32.to_rc_value(),5_i32.to_rc_value()]));
		return;
	    }
	    panic!("plus should be IFn, is: {:#?}",plus);
	}
    }
}

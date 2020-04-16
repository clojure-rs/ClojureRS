#[macro_use]
extern crate nom;

mod rust_core;
mod symbol;
mod type_tag;
mod value;
mod environment;
mod namespace;
mod ifn;
mod lambda;
mod reader;
mod persistent_list;
mod persistent_vector;

use environment::Environment;

use std::collections::HashMap;
use std::rc::Rc;
use std::io::{self,Read};
use std::str::FromStr;
use std::io::BufRead;
use std::fs::File;

use rust_core::{AddFn,StrFn};
use symbol::Symbol;
use crate::value::{ToValue,Evaluable};
use crate::persistent_list::{PersistentList,ToPersistentList};
use crate::persistent_vector::{PersistentVector,ToPersistentVector};
use crate::value::Value;

fn main()
{
    // Register our macros / functions ahead of time
    let add_fn = rust_core::AddFn{};
    let str_fn = rust_core::StrFn{};
    let do_fn = rust_core::DoFn{};
    let nth_fn = rust_core::NthFn{};
    let do_macro = rust_core::DoMacro{};
    let concat_fn = rust_core::ConcatFn{};
    // Hardcoded macros
    let let_macro = Value::LetMacro{};
    let quote_macro = Value::QuoteMacro{};
    let def_macro = Value::DefMacro{};
    let fn_macro = Value::FnMacro{};
    let defmacro_macro = Value::DefmacroMacro{};
    
    let environment = Rc::new(Environment::new_main_environment());
    
    let eval_fn = rust_core::EvalFn::new(Rc::clone(&environment));
    
    environment.insert(Symbol::intern("+"),add_fn.to_rc_value());
    environment.insert(Symbol::intern("let"),let_macro.to_rc_value());
    environment.insert(Symbol::intern("str"),str_fn.to_rc_value());
    environment.insert(Symbol::intern("quote"),quote_macro.to_rc_value());
    environment.insert(Symbol::intern("do-fn*"),do_fn.to_rc_value());
    environment.insert(Symbol::intern("do"),do_macro.to_rc_value());
    environment.insert(Symbol::intern("def"),def_macro.to_rc_value());
    environment.insert(Symbol::intern("fn"),fn_macro.to_rc_value());
    environment.insert(Symbol::intern("defmacro"),defmacro_macro.to_rc_value());
    environment.insert(Symbol::intern("eval"),eval_fn.to_rc_value());
    environment.insert(Symbol::intern("nth"),nth_fn.to_rc_value());
    environment.insert(Symbol::intern("concat"),concat_fn.to_rc_value());
    //
    // Start repl 
    //
    let stdin = io::stdin();
    print!("user=> ");
    for line in stdin.lock().lines() {
	let line = line.unwrap();
	let mut remaining_input = line.as_bytes();
	loop {
	    let next_read_parse = reader::try_read(remaining_input);
	    match next_read_parse {
		Ok((_remaining_input,value)) => {
		    print!("{} ",value.eval(Rc::clone(&environment)).to_string_explicit());
		    remaining_input = _remaining_input;
		},
		_ => {
		    break;
		}
	    }
	}
	println!();
	print!("user=> ");
    }
    
}




































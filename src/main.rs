#[macro_use]
extern crate nom;

mod environment;
mod ifn;
mod lambda;
mod maps;
mod namespace;
mod persistent_list;
mod persistent_list_map;
mod persistent_vector;
mod reader;
mod repl;
mod rust_core;
mod symbol;
mod type_tag;
mod value;

use environment::Environment;

use std::io::BufRead;
use std::io::{self};
use std::io::Write;
use std::rc::Rc;

use crate::value::Value;
use crate::value::{Evaluable, ToValue};
use symbol::Symbol;

use nom::Err::Incomplete;

fn main() {
    //
    // Start repl
    //
    repl::repl();
}

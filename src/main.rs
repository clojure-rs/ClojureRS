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
mod error_message;

fn main() {
    //
    // Start repl
    //
    repl::repl();
}

#[macro_use]
extern crate nom;
extern crate itertools;

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
mod util;
mod value;

fn main() {
    //
    // Start repl
    //
    let repl = repl::Repl::default();
    repl.run();
}

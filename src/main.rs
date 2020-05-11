#[macro_use]
extern crate nom;
extern crate itertools;

mod clojure_std;
mod environment;
mod error_message;
mod ifn;
mod keyword;
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
mod protocol;
mod iterable;

fn main() {
    //
    // Start repl
    //
    let repl = repl::Repl::default();
    repl.run();
}

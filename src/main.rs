#[macro_use]
extern crate nom;
extern crate itertools;

mod clojure_std;
mod clojure_string;
mod environment;
mod error_message;
mod ifn;
mod iterable;
mod keyword;
mod lambda;
mod maps;
mod namespace;
mod persistent_list;
mod persistent_list_map;
mod persistent_vector;
#[macro_use]
mod protocol;
mod reader;
mod repl;
mod rust_core;
mod symbol;
mod type_tag;
mod user_action;
mod util;
mod value;

mod protocols;
fn main() {
    let cli_args: user_action::Action = user_action::parse_args(std::env::args().collect());

    // instantiate the core environment
    let repl = repl::Repl::default();

    match cli_args {
        // eval the file/script
        user_action::Action::RunScript(script) => {
            println!("{}", repl::Repl::eval_file(&repl, script.as_str()));
        }

        // eval the expression
        user_action::Action::Evaluate(expression) => {
            println!(
                "{}",
                repl::Repl::eval(&repl, &repl::Repl::read_string(&expression))
            );
        }

        // Start repl
        user_action::Action::Nothing => {
            repl.run();
        }
    }
}

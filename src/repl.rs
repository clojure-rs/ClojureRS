use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::environment::Environment;
use crate::reader;
use crate::value::Evaluable;
use crate::value::Value;
use std::rc::Rc;

pub struct Repl {
    environment: Rc<Environment>,
}
impl Repl {
    pub fn new(environment: Rc<Environment>) -> Repl {
        Repl { environment }
    }

    // @TODO reconsider eval's signature;  since Value wraps all evaluables,  it might make more sense
    // to frame eval as "environment.eval(value)", and then likewise define a
    // 'repl.eval(value)', rather than 'value.eval(environment)'
    pub fn eval(&self, value: &Value) -> Value {
        value.eval(Rc::clone(&self.environment))
    }

    // Just wraps reader's read
    pub fn read<R: BufRead>(reader: &mut R) -> Value {
        reader::read(reader)
    }
    pub fn run(&self) {
        let stdin = io::stdin();
        let mut stdin_reader = stdin.lock();

        loop {
            print!("{}=> ",self.environment.get_current_namespace_name());
            let _ = io::stdout().flush();

            // Read
            let mut next = Repl::read(&mut stdin_reader);
            // Eval
            let evaled_next = self.eval(&next);
            // Print
            println!("{}", evaled_next);
            // Loop
        }
    }
    //
    // Will possibly just add this to our environment, or turn this into a parallel of clojure.lang.RT
    //
    /// Reads the code in a file sequentially and evaluates the result
    pub fn try_eval_file(&self, filepath: &str) -> Result<Value, std::io::Error> {
        let core = File::open(filepath)?;
        let mut reader = BufReader::new(core);

        let mut last_val = Repl::read(&mut reader);
        loop {
            // @TODO this is hardcoded until we refactor Conditions to have keys, so that
            //       we can properly identify them
            // @FIXME
            if let Value::Condition(cond) = &last_val {
                if cond != "Tried to read empty stream; unexpected EOF" {
                    println!("Error reading file: {}", cond);
                }

                return Ok(last_val);
            }

            let evaled_last_val = self.eval(&last_val);

            if let Value::Condition(cond) = evaled_last_val {
                println!("{}", cond);
            }

            last_val = Repl::read(&mut reader);
        }
    }
}

impl Default for Repl {
    fn default() -> Repl {
        Repl {
            environment: Environment::clojure_core_environment(),
        }
    }
}

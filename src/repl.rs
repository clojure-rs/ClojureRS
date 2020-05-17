use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use crate::environment::Environment;
use crate::reader;
use crate::value::{Evaluable, ToValue, Value};
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
    // @TODO add to reader.rs and wrap here
    pub fn read_string(string: &str) -> Value {
        Repl::read(&mut string.as_bytes())
    }
    pub fn run(&self) {
        let stdin = io::stdin();

        loop {
            print!("{}=> ", self.environment.get_current_namespace_name());
            let _ = io::stdout().flush();

            let next = {
                let mut stdin_reader = stdin.lock();
                // Read
                Repl::read(&mut stdin_reader)
                // Release stdin.lock
            };

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
    pub fn eval_file(&self, filepath: &str) -> Value {
        self.try_eval_file(filepath).to_value()
    }
}

impl Default for Repl {
    fn default() -> Repl {
        Repl {
            environment: Environment::clojure_core_environment(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repl::Repl;
    use crate::value::Value;
    //@TODO separate into individual tests
    #[test]
    fn read_string() {
        let num = Repl::read_string("1");
        match num {
            Value::I32(_) => {}
            _ => panic!("Reading of integer should have returned Value::I32"),
        }
        let list = Repl::read_string("(+ 1 2)");
        match list {
            Value::PersistentList(_) => {}
            _ => panic!("Reading of integer should have returned Value::PersistentList"),
        }

        let vector = Repl::read_string("[1 2 a]");
        match vector {
            Value::PersistentVector(_) => {}
            _ => panic!("Reading of integer should have returned Value::PersistentVector"),
        }

        let symbol = Repl::read_string("abc");
        match symbol {
            Value::Symbol(_) => {}
            _ => panic!("Reading of integer should have returned Value::Symbol"),
        }
    }
}

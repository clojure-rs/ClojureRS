use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self};
use std::io::Write;

use crate::environment::Environment;
use crate::reader;
use crate::value::Evaluable;
use crate::value::Value;
use std::rc::Rc;

use nom::Err::Incomplete;
use nom::Needed::Size;

//
// Will possibly just add this to our environment, or turn this into a parallel of clojure.lang.RT
//
pub fn try_eval_file(environment: &Rc<Environment>, filepath: &str) -> Result<(), io::Error> {
    let core = File::open(filepath)?;

    let reader = BufReader::new(core);

    let mut remaining_input_buffer = String::from("");
    for line in reader.lines() {
        let line = line?;
        remaining_input_buffer.push_str(&line);
        let mut remaining_input_bytes = remaining_input_buffer.as_bytes();
        loop {
            let next_read_parse = reader::try_read(remaining_input_bytes);
            match next_read_parse {
                Ok((_remaining_input, value)) => {
                    //print!("{} ",value.eval(Rc::clone(&environment)).to_string_explicit());
                    value.eval(Rc::clone(&environment));
                    remaining_input_bytes = _remaining_input;
                }
                Err(Incomplete(Size(1))) => {
                    remaining_input_buffer =
                        String::from_utf8(remaining_input_bytes.to_vec()).unwrap();
                    break;
                }
                err => {
                    println!(
                        "Error evaluating file {}; {}",
                        filepath,
                        Value::Condition(format!("Reader Error: {:?}", err))
                    );
                    remaining_input_buffer = String::from("");
                    break;
                }
            }
        }
    }

    Ok(())
}
// @TODO eventually, this will likely be implemented purely in Clojure
/// Starts an entirely new session of Clojure RS 
pub fn repl() {
    println!("Clojure RS 0.0.1");

    let environment = Environment::clojure_core_environment();
    let stdin = io::stdin();

    print!("user=> ");
    let _ = io::stdout().flush();
    let mut remaining_input_buffer = String::from("");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        remaining_input_buffer.push_str(&line);
        let mut remaining_input_bytes = remaining_input_buffer.as_bytes();
        loop {
            let next_read_parse = reader::try_read(remaining_input_bytes);
            match next_read_parse {
                Ok((_remaining_input_bytes, value)) => {
                    print!(
                        "{} ",
                        value.eval(Rc::clone(&environment)).to_string_explicit()
                    );
                    remaining_input_bytes = _remaining_input_bytes;
                }
                Err(Incomplete(_)) => {
                    remaining_input_buffer =
                        String::from_utf8(remaining_input_bytes.to_vec()).unwrap();
                    break;
                }
                err => {
                    print!("{}", Value::Condition(format!("Reader Error: {:?}", err)));
                    remaining_input_buffer = String::from("");
                    break;
                }
            }
        }
        println!();
        print!("user=> ");
	let _ = io::stdout().flush();
    }
}

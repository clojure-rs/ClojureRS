use std::fs::File;
use std::io::{self,Read};
use std::str::FromStr;
use std::io::BufRead;
use std::io::BufReader;

use crate::reader;
use crate::environment::Environment;
use crate::value::Value;
use crate::value::Evaluable;
use std::rc::Rc;

use nom::Err::Incomplete;
use nom::error::convert_error;
use nom::Needed::Size;

//
// Will possibly just add this to our environment, or turn this into a parallel of clojure.lang.RT 
//

pub fn try_eval_file(environment: &Rc<Environment>,filepath: &str) -> Result<(),io::Error>{
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
		Ok((_remaining_input,value)) => {
		    //print!("{} ",value.eval(Rc::clone(&environment)).to_string_explicit());
		    value.eval(Rc::clone(&environment));
		    remaining_input_bytes = _remaining_input;
		},
		Err(Incomplete(Size(1))) => {
		    remaining_input_buffer = String::from_utf8(remaining_input_bytes.to_vec()).unwrap();
		    break;
		},
		err => {
		    println!("Error evaluating file {}; {}",filepath,Value::Condition(format!("Reader Error: {:?}",err)));
		    remaining_input_buffer = String::from("");
		    break;
		}
	    }
	}
    }

    Ok(())
    
}

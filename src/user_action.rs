use std::env;
use std::fmt;

pub enum Action{
    RunScript(String),
    Evaluate(String),
    Nothing,	
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &*self {
	   Action::RunScript(filepath) => write!(f, "RunScript"),
	   Action::Evaluate(expression) => write!(f, "Evaluate"),
	   Action::Nothing => write!(f, "Nothing"),
       }
    }
}

pub fn parse_args() -> Action{
    
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 3 {
	if arguments[1] == "-i" || arguments[1] == "--init" {
	    return Action::RunScript(arguments[2].clone());
	}else if arguments[1] == "-e" || arguments[1] == "--eval" {
	    return Action::Evaluate(arguments[2].clone());
	}else { return Action::Nothing; }
    }else {
	return Action::Nothing;
    }	    
}
    

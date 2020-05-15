use std::fmt;
use crate::user_action::Action::*;

pub enum Action{
    RunScript(String),
    Evaluate(String),
    Nothing,	
}

impl PartialEq for Action {
    fn eq(&self, other: &Action) -> bool{
	match (self, other) {
	    (RunScript(script1), RunScript(script2)) => script1 == script2,
	    (Evaluate(expression1), Evaluate(expression2)) => expression1 == expression2,
	    (Nothing, Nothing) => true,
	    _ => false,
	}
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &*self {
	   Action::RunScript(filepath) => write!(f, "RunScript: {}", filepath),
	   Action::Evaluate(expression) => write!(f, "Evaluate: {}", expression),
	   Action::Nothing => write!(f, "Nothing"),
       }
    }
}

pub fn parse_args(arguments: Vec<String>) -> Action{
    
    if arguments.len() >= 2 {
	if arguments[1] == "-i" || arguments[1] == "--init" {
	    return Action::RunScript(arguments[2].clone());
	}else if arguments[1] == "-e" || arguments[1] == "--eval" {
	    return Action::Evaluate(arguments[2].clone());
	}else { Action::RunScript(arguments[1].clone()) } // for path as argument
    }else {
	return Action::Nothing;
    }	    
}
    
#[cfg(test)]
mod tests {
    use crate::user_action;

    #[test]
    fn parse_args_test() {
	let path = "target/debug/rust_clojure".to_string();
	
	assert_eq!(user_action::Action::RunScript("examples/hello_world.clj".to_string()),
		   user_action::parse_args(vec![path.clone(), "examples/hello_world.clj".to_string()]));

	assert_eq!(user_action::Action::RunScript("test.clj".to_string()),
		   user_action::parse_args(vec![path.clone(), "-i".to_string(), "test.clj".to_string()]));

	assert_eq!(user_action::Action::RunScript("testing.clj".to_string()),
		   user_action::parse_args(vec![path.clone(), "--init".to_string(), "testing.clj".to_string()]));

	assert_eq!(user_action::Action::Evaluate("(+ 1 2 3)".to_string()),
		   user_action::parse_args(vec![path.clone(), "-e".to_string(), "(+ 1 2 3)".to_string()]));

	assert_eq!(user_action::Action::Evaluate("(println \"eh\")".to_string()),
		   user_action::parse_args(vec![path.clone(), "--eval".to_string(), "(println \"eh\")".to_string()]));

	assert_eq!(user_action::Action::Nothing, user_action::parse_args(vec![path.clone()]));
    }
}

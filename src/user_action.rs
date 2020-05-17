use std::fmt;

#[derive(PartialEq, Eq)]
pub enum Action {
    RunScript(String),
    Evaluate(String),
    Nothing,
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

pub fn parse_args(arguments: Vec<String>) -> Action {
    if arguments.len() >= 2 {
        if arguments[1] == "-i" || arguments[1] == "--init" {
            return Action::RunScript(arguments[2].clone());
        } else if arguments[1] == "-e" || arguments[1] == "--eval" {
            return Action::Evaluate(arguments[2].clone());
        } else {
            Action::RunScript(arguments[1].clone())
        } // for path as argument
    } else {
        return Action::Nothing;
    }
}

#[cfg(test)]
mod tests {
    mod parse_args_test {
        use crate::user_action;

        #[test]
        fn parses_args_given_path() {
            let arguments = vec![
                "target/debug/rust_clojure".to_string(),
                "examples/hello_world.clj".to_string(),
            ];
            assert_eq!(
                user_action::Action::RunScript("examples/hello_world.clj".to_string()),
                user_action::parse_args(arguments)
            );
        }

        #[test]
        fn parses_args_given_i() {
            let arguments = vec![
                "target/debug/rust_clojure".to_string(),
                "-i".to_string(),
                "test.clj".to_string(),
            ];

            assert_eq!(
                user_action::Action::RunScript("test.clj".to_string()),
                user_action::parse_args(arguments)
            );
        }

        #[test]
        fn parses_args_given_init() {
            let arguments = vec![
                "target/debug/rust_clojure".to_string(),
                "--init".to_string(),
                "testing.clj".to_string(),
            ];

            assert_eq!(
                user_action::Action::RunScript("testing.clj".to_string()),
                user_action::parse_args(arguments)
            );
        }

        #[test]
        fn parses_args_given_e() {
            let arguments = vec![
                "target/debug/rust_clojure".to_string(),
                "-e".to_string(),
                "(+ 1 2 3)".to_string(),
            ];

            assert_eq!(
                user_action::Action::Evaluate("(+ 1 2 3)".to_string()),
                user_action::parse_args(arguments)
            );
        }

        #[test]
        fn parses_args_given_eval() {
            let arguments = vec![
                "target/debug/rust_clojure".to_string(),
                "--eval".to_string(),
                "(println \"eh\")".to_string(),
            ];

            assert_eq!(
                user_action::Action::Evaluate("(println \"eh\")".to_string()),
                user_action::parse_args(arguments)
            );
        }

        #[test]
        fn parses_args_given_nil() {
            assert_eq!(
                user_action::Action::Nothing,
                user_action::parse_args(vec!["target/debug/rust_clojure".to_string()])
            );
        }
    }
}

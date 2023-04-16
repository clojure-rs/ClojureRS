use crate::environment::Environment;
use crate::ifn::IFn;
use crate::persistent_list::ToPersistentList;
use crate::symbol::Symbol;
use crate::value::{Evaluable, ToValue, Value};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Fn {
    pub body: Rc<Value>,
    // Closed over variables
    pub enclosing_environment: Rc<Environment>,
    pub arg_syms: Vec<Symbol>,
}
impl ToValue for Fn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for Fn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        let local_environment = Rc::new(Environment::new_local_environment(Rc::clone(
            &self.enclosing_environment,
        )));

        let argc = self.arg_syms.len();

        let mut var_args = false;
        if argc >= 2 {
            if let Some(sym) = self.arg_syms.get(argc - 2) {
                if sym.to_string() == "&" {
                    var_args = true;
                    let last_sym = self.arg_syms.get(argc - 1).unwrap();
                    local_environment.insert(last_sym.clone(), Rc::new(Value::Nil));
                }
            }
        }

        if var_args && args.len() < argc -  2 {
            return Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: {} or more)",
                args.len(),
                argc - 2
            ));
        }
        if !var_args && args.len() != argc {
            return Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: {})",
                args.len(),
                argc
            ));
        }

        for (i, arg) in args.iter().enumerate() {
            let curr_sym = self.arg_syms.get(i).unwrap();
            // We can bind the rest of the arguments, then, to the next variable and blow this popsicle stand
            if curr_sym.to_string() == "&" {
                if !var_args {
                    return Value::Condition(String::from("Invalid function argument '&' in non-variable-argument function definition"));
                }
                let last_sym = self.arg_syms.get(i + 1).unwrap();
                let rest_args = args.get(i..).unwrap().to_vec().into_list().to_rc_value();
                local_environment.insert(last_sym.clone(), rest_args);
                break;
            }
            local_environment.insert(curr_sym.clone(), arg.to_rc_value());
        }
        self.body.eval(local_environment)
    }
}

///
/// Tests
///

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::environment::Environment;
    use crate::ifn::IFn;
    use crate::lambda;
    use crate::symbol::Symbol;
    use crate::value::Value;

    #[test]
    fn test_only_vararg() {
        // (defn func [& vararg] "Works")
        let func = lambda::Fn {
            body: Rc::new(Value::String(String::from("Works"))),
            enclosing_environment: Rc::new(Environment::new_local_environment(Environment::clojure_core_environment())),
            arg_syms: vec![Symbol::intern("&"), Symbol::intern("varargs")],
        };

        let val = func.invoke(vec![]); // (func)
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![Rc::new(Value::I32(1_i32))]); // (func 1)
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![    //
            Rc::new(Value::I32(1_i32)),      //  (func 1 2)
            Rc::new(Value::I32(2_i32)),      //
        ]);                                        //
        assert_eq!(val, Value::String("Works".to_string()));
    }

    #[test]
    fn test_vararg_one() {
        // (defn func [x & vararg] "Works")
        let func = lambda::Fn {
            body: Rc::new(Value::String(String::from("Works"))),
            enclosing_environment: Rc::new(Environment::new_local_environment(Environment::clojure_core_environment())),
            arg_syms: vec![Symbol::intern("x"), Symbol::intern("&"), Symbol::intern("varargs")],
        };

        let val = func.invoke(vec![]); // (func)
        assert_eq!(val, Value::Condition("Wrong number of arguments given to function (Given: 0, Expected: 1 or more)".to_string()));

        let val = func.invoke(vec![Rc::new(Value::I32(1_i32))]); // (func 1)
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![                                    //  (func 1 2)
                                       Rc::new(Value::I32(1_i32)),
                                       Rc::new(Value::I32(2_i32)),
        ]);
        assert_eq!(val, Value::String("Works".to_string()));
    }

    #[test]
    fn test_vararg_two() {
        // (defn func [x y & vararg] "Works")
        let func = lambda::Fn {
            body: Rc::new(Value::String(String::from("Works"))),
            enclosing_environment: Rc::new(Environment::new_local_environment(Environment::clojure_core_environment())),
            arg_syms: vec![Symbol::intern("x"), Symbol::intern("y"),
                           Symbol::intern("&"), Symbol::intern("varargs")],
        };

        let val = func.invoke(vec![]); // (func)
        assert_eq!(val, Value::Condition("Wrong number of arguments given to function (Given: 0, Expected: 2 or more)".to_string()));

        let val = func.invoke(vec![Rc::new(Value::I32(1_i32))]); // (func 1)
        assert_eq!(val, Value::Condition("Wrong number of arguments given to function (Given: 1, Expected: 2 or more)".to_string()));

        let val = func.invoke(vec![ //  (func 1 2)
            Rc::new(Value::I32(1_i32)),
            Rc::new(Value::I32(2_i32)),
        ]);
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![ //  (func 1 2 "vararg here")
            Rc::new(Value::I32(1_i32)),
            Rc::new(Value::I32(2_i32)),
            Rc::new(Value::String(String::from("vararg here"))),
        ]);
        assert_eq!(val, Value::String("Works".to_string()));
    }
}


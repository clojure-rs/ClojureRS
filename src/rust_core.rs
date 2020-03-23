//use crate::ifn::IFn;
use crate::value::Value;
use crate::value::{IFn,ToValue};
/*
This module will hold the core functions and macros that Clojure will
hook into; Functions / Macros like "+", "fn*", "let", "cond", etc

This is still experimental, and we may instead undo this all and
represent fn* and let and the like the same it is done in ClojureJVM,
where I believe they are defined flat out in the Compiler class

However, even in that case this may stick around to implement basic
functions like "+" that usually rely on interop

*/

/*
{:clojure-equivalent '+,
 :arglists '([name doc-string? attr-map? [params*] body]
                 [name doc-string? attr-map? ([params*] body)+ attr-map?]) 
 */
#[derive(Debug)]
pub struct AddFn {
    

}
impl IFn for AddFn {
    fn invoke(&self,args: &[&Value]) -> Value {
	args.into_iter().fold(0_i32.to_value(),|a,b|  {
	    match a {
		Value::I32(a_) => match b {
		    Value::I32(b_) =>  Value::I32(a_ + b_),
		    // @TODO insert actual value of b and type into error message 
			_ =>  Value::Condition(String::from("Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: "))
		}
		// @TODO insert actual value of b and type into error message 
		_ => Value::Condition(String::from("Type mismatch: Expecting: (i32 | i64 | f32 | f64), Found: "))
		      
	    }
	})
    }
}

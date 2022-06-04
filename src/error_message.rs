use crate::type_tag::TypeTag;
use crate::value::Value;
use std::error::Error;
use std::fmt;

pub fn type_mismatch(expected: TypeTag, got: &Value) -> Value {
    Value::Condition(format!(
        "Type mismatch; Expected instance of {},  Recieved type {}",
        expected, got
    ))
}

pub fn wrong_arg_count(expected: usize, got: usize) -> Value {
    Value::Condition(format!(
        "Wrong number of arguments given to function (Given: {}, Expected: {})",
        got, expected
    ))
}

pub fn wrong_varg_count(expected: &[usize], got: usize) -> Value {
    Value::Condition(format!(
        "Wrong number of arguments given to function (Given: {}, Expected: {:?})",
        got, expected
    ))
}

pub fn zero_arg_count(got: usize) -> Value {
    Value::Condition(format!(
        "Wrong number of arguments given to function (Given: {})",
        got
    ))
}

pub fn index_out_of_bounds(ind: usize, count: usize) -> Value {
    Value::Condition(format!(
        "Index out of bounds: Index ({}), Length: ({})",
        ind, count
    ))
}

pub fn index_cannot_be_negative(ind: usize) -> Value {
    Value::Condition(format!("Index cannot be negative; Index ({})", ind))
}

pub fn generic_err(error: Box<dyn Error>) -> Value {
    Value::Condition(error.to_string())
}

pub fn unknown_err(error: String) -> Value {
    Value::Condition(error)
}
//
// This module will likely be rewritten to look like everything below this line
// The general idea is that for any submessage in an error that is truly 'variable', like
// one that's sometimes:
//
// "Type mismatch: Expected an [Int]"
//
// And sometimes
//
// "Type mismatch: Expected a [Float that is greater than 12 but sometimes 5]"
//
// we cannot express this cleanly with just one pre-existing type, we need to create a new custom type
// (or, if we wish to be a bit more flexible and basically forgo the type system, we can use a plain string).
// In my case, I think really expressing either as either is fine, as long as the same error produces
// the same message each time, which can be enforced with functions; ie
//
// pub weird_error() -> String {
//    "This always returns the same weird error"
// }
//
// I don't forsee this the sort of thing for a bug to hide in, and one that requires strong type guarantees
//

// We currently don't have any type that represents an interface type name, so we
pub struct Cast<'a>(pub &'a str);
pub fn cast(expected: Cast, found: TypeTag) -> Value {
    Value::Condition(format!("Cannot cast {} to {}", found, expected.0))
}

/// For one off errors
pub fn custom(message: &str) -> Value {
    Value::Condition(String::from(message))
}

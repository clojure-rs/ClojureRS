use crate::type_tag::TypeTag;
use crate::value::Value;
use std::error::Error;

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

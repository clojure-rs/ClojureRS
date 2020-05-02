use crate::type_tag::TypeTag;
use crate::value::Value;

pub fn type_mismatch(expected: TypeTag, got: Option<&&Value>) -> Value {
    let received_type = if got.is_some() { got.unwrap().type_tag() } else { TypeTag::Nil };
    Value::Condition(format!(
        "Type mismatch; Expected instance of {},  Recieved type {}",
        TypeTag::Integer,
        received_type
    ))
}

pub fn wrong_arg_count(expected: usize, got: usize) -> Value {
    Value::Condition(format!(
        "Wrong number of arguments given to function (Given: {}, Expected: {})",
        got,
        expected
    ))
}

pub fn wrong_varg_count(expected: &[usize], got: usize) -> Value {
    Value::Condition(format!(
        "Wrong number of arguments given to function (Given: {}, Expected: {:?})",
        got,
        expected
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
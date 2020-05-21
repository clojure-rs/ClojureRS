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

pub fn invalid_key<T: fmt::Display>(given: &T, valid: &[T]) -> Value {
    Value::Condition(format!(
        "Invalid key; given: {}, expecting one of ({})",
        given,
        valid
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    ))
}

// pub fn custom(error: &str) -> Value {
//     Value::Condition(String::from(error))
// }

// /// error_message::strict::..
// /// Are a set of error message tools that are very rigid in how
// /// you can build error messages, leveraging the type system
// /// to write very specific errors.  For a more flexible
// /// set of tools, set error_message::free
// mod free {
//     use crate::type_tag::TypeTag;
//     use crate::value::Value;

//     enum ErrorMessage {
//         // "Expected: clojure.lang.string"
//         Type(TypeTag),
//         // "Expected: 1"
//         Size(usize),
//         // "Expected: I32 | F32 | I64"
//         TypeRange(Vec<TypeTag>),
//         Custom(String)
//     }
//     pub fn type_mismatch(expected: &str, got: &str) -> Value {
//         Value::Condition(format!(
//             "Type mismatch; Expected instance of {},  Recieved type {}",
//             expected, got
//         ))
//     }
//     pub fn wrong_arg_count(expected: &str, got: &str) -> Value {
//         Value::Condition(format!(
//             "Wrong number of arguments given to function (Given: {}, Expected: {})",
//             got, expected
//         ))
//     }

//     pub fn index_out_of_bounds(ind: usize, count: usize) -> Value {
//         Value::Condition(format!(
//             "Index out of bounds: Index ({}), Length: ({})",
//             ind, count
//         ))
//     }

//     pub fn index_cannot_be_negative(ind: usize) -> Value {
//         Value::Condition(format!("Index cannot be negative; Index ({})", ind))
//     }

//     // pub fn generic_err(error: Box<dyn Error>) -> Value {
//     //     Value::Condition(error.to_string())
//     // }

//     pub fn custom(error: &str) -> Value {
//         Value::Condition(String::from(error))
//     }

//

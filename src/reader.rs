//! The reader.  The part that reads plain text and parses it into Clojure structures, which are
//! themselves code.
//!
//! Right now there's no sort of data kept track by the reader at any point, so there's no real
//! reader data structure here -- this is just a plain module, a bag of functions.  However,
//! I believe this will change -- especially as, for instance, we define the idea of reader conditionals,
//! or even reader macros,  although the latter will likely be reserved for our interpreter here (but perhaps
//! not;  since this is about being a 'free-er' Clojure, especially since it can't compete with it in raw
//! power, neither speed or ecosystem,  it might be worth it to leave in reader macros.
extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::multispace0,
    character::is_digit,
    character::{is_alphabetic, is_alphanumeric},
    combinator::map_res,
    error::convert_error,
    sequence::{preceded, terminated},
    IResult,
};

use crate::maps::MapEntry;
use crate::persistent_list::ToPersistentList;
use crate::persistent_list_map::{PersistentListMap, ToPersistentListMap};
use crate::persistent_vector::ToPersistentVector;
use crate::symbol::Symbol;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use std::fs::File;
//
// Note; the difference between ours 'parsers'
//   identifier_parser
//   symbol_parser
//   integer_parser
// And our 'try readers'
//   try_read_i32
//   try_read_string
//   try_read_map
//   try_read_list
//   try_read_vector
//
// Is our parsers are meant to be be nom parsers, and more primitive in that
// they can parse any information that we can later use to create a value::Value
// 
// Our 'try readers' are a bit higher level, and are specifically supposed to be returning a valid // value::Value or some sort of failure.
//

/// Parses valid Clojure identifiers
/// Example Successes: ab,  cat,  -12+3, |blah|, <well>
/// Example Failures:  'a,  12b,   ,cat  
pub fn identifier_parser(input: &[u8]) -> IResult<&[u8], String> {
    named!( non_numeric_identifier_char<&[u8],u8>,
	    alt!( map!(one_of!("|?<>+-_=^%&$*!"), |x| x as u8 ) |
		  map!(take_while_m_n!(1,1,is_alphabetic),|ls| ls[0] as u8)));
    named!( identifier_char<&[u8],u8>,
	    alt!( map!(one_of!("|?<>+-_=^%&$*!"), |x| x as u8 ) |
		  map!(take_while_m_n!(1,1,is_alphanumeric),|ls| ls[0] as u8)));
    named!( identifier_ <&[u8],String> ,
    do_parse!(
    head: non_numeric_identifier_char >>
    rest_input:
    map_res!(
        many0!(complete!(identifier_char)),
        String::from_utf8) >>
    (format!("{}{}",head as char,rest_input))
    ));

    identifier_(input)
}

/// Parses valid Clojure symbols,  whose name is a valid identifier
pub fn symbol_parser(input: &[u8]) -> IResult<&[u8], Symbol> {
    identifier_parser(input).map(|(rest_input, name)| (rest_input, Symbol::intern(&name)))
}

// @TODO add negatives
/// Parses valid integers
/// Example Successes: 1, 2, 4153,  -12421
pub fn integer_parser(input: &[u8]) -> IResult<&[u8], i32> {
    map_res(take_while1(is_digit), |digits: &[u8]| {
        String::from_utf8(digits.to_vec()).map(|digit_string| digit_string.parse::<i32>().unwrap())
    })(input)
}
// Currently used to create 'try_readers', which are readers (or
// reader functions, at least) that are basically composable InputType
// -> IResult<InputType,Value> parsers, that our normal read function
// / reader will wrap.
/// Takes a parser, such as one that reads a &[u8] and returns an
/// i32, and creates a new parser that instead returns a valid
/// ClojureRS Value instead
pub fn to_value_parser<I, O: ToValue>(
    parser: impl Fn(I) -> IResult<I, O>,
) -> impl Fn(I) -> IResult<I, Value> {
    move |input: I| parser(input).map(|(rest_input, thing)| (rest_input, thing.to_value()))
}

// @TODO make sure whitespace or 'nothing' is at the end, fail for
// float like numbers
/// Tries to parse &[u8] into Value::I32
/// Expects:
///   Integers
/// Example Successes:
///    1 => Value::I32(1),
///    5 => Value::I32(5),
///    1231415 => Value::I32(1231415)
/// Example Failures:
///    1.5,  7.1321 , 1423152621625226126431525
pub fn try_read_i32(input: &[u8]) -> IResult<&[u8], Value> {
    to_value_parser(integer_parser)(input)
}

/// Tries to parse &[u8] into Value::Symbol
/// Example Successes:
///    a                    => Value::Symbol(Symbol { name: "a" })
///    cat-dog              => Value::Symbol(Symbol { name: "cat-dog" })
///    +common-lisp-global+ => Value::Symbol(Symbol { name: "+common-lisp-global+" })
/// Example Failures:
///    12cat,  'quoted,  @at-is-for-references
pub fn try_read_symbol(input: &[u8]) -> IResult<&[u8], Value> {
    to_value_parser(symbol_parser)(input)
}

// @TODO allow escaped strings
/// Tries to parse &[u8] into Value::String
/// Example Successes:
///    "this is pretty straightforward" => Value::String("this is pretty straightforward")
pub fn try_read_string(input: &[u8]) -> IResult<&[u8], Value> {
    named!(
        quotation,
        preceded!(consume_clojure_whitespaces, tag!("\""))
    );
    let (rest_input, _) = quotation(input)?;
    to_value_parser(map_res(
        terminated(take_until("\""), tag("\"")),
        |bytes: &[u8]| String::from_utf8(bytes.to_vec()),
    ))(rest_input)
}

// @TODO Perhaps generalize this, or even generalize it as a reader macro
/// Tries to parse &[u8] into Value::PersistentListMap, or some other Value::..Map   
/// Example Successes:
///    {:a 1} => Value::PersistentListMap {PersistentListMap { MapEntry { :a, 1} .. ]})
pub fn try_read_map(input: &[u8]) -> IResult<&[u8], Value> {
    named!(lbracep, preceded!(consume_clojure_whitespaces, tag!("{")));
    named!(rbracep, preceded!(consume_clojure_whitespaces, tag!("}")));
    let (map_inner_input, _) = lbracep(input)?;
    let mut map_as_vec: Vec<MapEntry> = vec![];
    let mut rest_input = map_inner_input;
    loop {
        let right_brace = rbracep(rest_input);
        match right_brace {
            Ok((after_map_input, _)) => {
                break Ok((after_map_input, map_as_vec.into_list_map().to_value()));
            }
            _ => {
                let (_rest_input, next_key) = try_read(rest_input)?;
                let (_rest_input, next_val) = try_read(_rest_input)?;
                map_as_vec.push(MapEntry {
                    key: Rc::new(next_key),
                    val: Rc::new(next_val),
                });
                rest_input = _rest_input;
            }
        }
    }
}

// @TODO use nom functions in place of macro
/// Tries to parse &[u8] into Value::PersistentVector
/// Example Successes:
///    [1 2 3] => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) ... ]})
///    [1 2 [5 10 15] 3]
///      => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) .. Rc(Value::PersistentVector..)]})
pub fn try_read_vector(input: &[u8]) -> IResult<&[u8], Value> {
    named!(lbracketp, preceded!(consume_clojure_whitespaces, tag!("[")));
    named!(rbracketp, preceded!(consume_clojure_whitespaces, tag!("]")));
    let (vector_inner_input, _) = lbracketp(input)?;
    let mut vector_as_vec = vec![];
    // What's left of our input as we read more of our PersistentVector
    let mut rest_input = vector_inner_input;
    loop {
        // Try parse end of vector
        let right_paren = rbracketp(rest_input);
        match right_paren {
            // If we succeeded,  we can convert our vector of values into a PersistentVector and return our success
            Ok((after_vector_input, _)) => {
                break Ok((after_vector_input, vector_as_vec.into_vector().to_value()));
            }
            // Otherwise, we need to keep reading until we get that closing bracket letting us know we're finished
            _ => {
                let next_form_parse = try_read(rest_input);
                match next_form_parse {
                    // Normal behavior;  read our next element in the PersistentVector
                    Ok((_rest_input, form)) => {
                        vector_as_vec.push(form.to_rc_value());
                        rest_input = _rest_input;
                    }
                    // This parse failed, return overall read failure
                    _ => {
                        break next_form_parse;
                    }
                }
            }
        }
    }
}

pub fn try_read_list(input: &[u8]) -> IResult<&[u8], Value> {
    named!(lparenp, preceded!(consume_clojure_whitespaces, tag!("(")));
    named!(rparenp, preceded!(consume_clojure_whitespaces, tag!(")")));

    let (list_inner_input, _) = lparenp(input)?;
    let mut list_as_vec = vec![];
    let mut rest_input = list_inner_input;
    loop {
        let right_paren = rparenp(rest_input);
        match right_paren {
            Ok((after_list_input, _)) => {
                break Ok((after_list_input, list_as_vec.into_list().to_value()));
            }
            _ => {
                let next_form_parse = try_read(rest_input);
                match next_form_parse {
                    Ok((_rest_input, form)) => {
                        list_as_vec.push(form.to_rc_value());
                        rest_input = _rest_input;
                    }
                    // This parse failed, forward failure
                    _ => {
                        break next_form_parse;
                    }
                }
            }
        }
    }
}

pub fn try_read(input: &[u8]) -> IResult<&[u8], Value> {
    preceded(
        multispace0,
        alt((
            try_read_map,
            try_read_string,
            try_read_symbol,
            try_read_i32,
            try_read_list,
            try_read_vector,
        )),
    )(input)
}

pub fn debug_try_read(input: &[u8]) -> IResult<&[u8], Value> {
    let reading = try_read(input);
    match &reading {
        Ok((_, value)) => println!("Reading: {}", value),
        _ => println!("Reading: {:?}", reading),
    };
    reading
}

/// Consumes one or more whitespaces from the input.
///
/// A whitespace is either an ASCII whitespace or a comma.
fn consume_clojure_whitespaces(input: &[u8]) -> IResult<&[u8], ()> {
    named!(parser, take_while!(is_clojure_whitespace));
    parser(input).map(|(rest, _)| (rest, ()))
}

/// Returns whether if a given character is a whitespace.
///
/// Clojure defines a whitespace as either a comma or an ASCII whitespace.
fn is_clojure_whitespace(c: u8) -> bool {
    // ASCII symbol of `,` is 44.
    c.is_ascii_whitespace() || c == 44
}

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
    IResult,
    branch::alt,
    character::{is_alphabetic,is_alphanumeric},
    character::complete::multispace0,
    character::is_digit,
    bytes::complete::{take_while1,take_until},
    combinator::map_res,
    sequence::preceded};

use crate::value::{Value,ToValue};
use crate::persistent_list::{ToPersistentList};
use crate::persistent_vector::{ToPersistentVector};
use crate::symbol::Symbol;

/// Parses valid Clojure identifiers
/// Example Successes: ab,  cat,  -12+3, |blah|, <well>
/// Example Failures:  'a,  12b,   ,cat  
pub fn identifier_parser(input:&[u8]) -> IResult<&[u8], String> {
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
    identifier_parser(input).map(|(rest_input,name)| {
	(rest_input, Symbol::intern(&name))
    })
}

// @TODO add negatives 
/// Parses valid integers
/// Example Successes: 1, 2, 4153,  -12421
pub fn integer(input: &[u8]) -> IResult<&[u8],i32> {
    map_res(take_while1(is_digit),|digits: &[u8]| { 
	String::from_utf8(digits.to_vec()).map(|digit_string| {
	    digit_string.parse::<i32>().unwrap()
	})
    })(input)	
}
// Currently used to create 'try_readers', which are readers (or
// reader functions, at least) that are basically composable InputType
// -> IResult<InputType,Value> parsers, that our normal read function
// / reader will wrap.
/// Takes a parser, such as one that reads a &[u8] and returns an
/// i32, and creates a new parser that instead returns a valid
/// ClojureRS Value instead 
pub fn to_value_parser<I,O: ToValue>(parser: impl Fn(I) -> IResult<I,O>) -> impl Fn(I) -> IResult<I,Value> {
    move |input: I| parser(input).map(|(rest_input,thing)| (rest_input,thing.to_value()))
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
pub fn try_read_i32(input: &[u8]/*changed*/) -> IResult<&[u8],Value> {
    to_value_parser(integer)(input)
}

/// Tries to parse &[u8] into Value::Symbol
/// Example Successes:
///    a                    => Value::Symbol(Symbol { name: "a" })
///    cat-dog              => Value::Symbol(Symbol { name: "cat-dog" })
///    +common-lisp-global+ => Value::Symbol(Symbol { name: "+common-lisp-global+" })
/// Example Failures:
///    12cat,  'quoted,  @at-is-for-references 
pub fn try_read_symbol(input: &[u8]/*changed*/) -> IResult<&[u8],Value> {
    to_value_parser(symbol_parser)(input)
}
// @TODO allow escaped strings 
/// Tries to parse &[u8] into Value::String
/// Example Successes:
///    "this is pretty straightforward" => Value::String("this is pretty straightforward")
pub fn try_read_string(input: &[u8]/*changed*/) -> IResult<&[u8],Value> {
    named!(quotation,
	   ws!(tag!("\"")));
    let (rest_input,_) = quotation(input)?;
    to_value_parser(
	map_res(
	    take_until("\""),
	    |bytes: &[u8]| String::from_utf8(bytes.to_vec())))(rest_input)
}
// @TODO remove ws!, use nom functions in place of macro 
/// Tries to parse &[u8] into Value::PersistentVector 
/// Example Successes:
///    [1 2 3] => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) ... ]})
///    [1 2 [5 10 15] 3]
///      => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) .. Rc(Value::PersistentVector..)]})
pub fn try_read_vector(input: &[u8]/*changed*/) -> IResult<&[u8],Value> {
    named!(lbracketp,
	   ws!(tag!("[")));
    named!(rbracketp,
	   ws!(tag!("]")));
    // Consume [ 
    let (vector_inner_input,_) = lbracketp(input)?;
    // We will read all members of our PersistentVector and save them here 
    let mut vector_as_vec = vec![];
    // What's left of our input as we read more of our PersistentVector 
    let mut rest_input = vector_inner_input;
    loop {
	// We're gonna try to consume a closing bracket 
	let right_paren = rbracketp(rest_input);
	match right_paren {
	    // If we succeeded,  we can convert our vector of values in a PersistentVector and return our success
	    Ok((after_vector_input,_)) => {
		break Ok((after_vector_input,vector_as_vec.into_vector().to_value()));
	    },
	    // Otherwise, we need to keep reading until we get a closing bracket letting us know we're finished
	    _ => {
		// Read next .. anything 
		let next_form_parse = try_read(rest_input);
		match next_form_parse {
		    // Normal behavior;  read our next element in the PersistentVector
		    Ok((_rest_input,form)) => 	{
			vector_as_vec.push(form.to_rc_value());
			rest_input = _rest_input;
		    },
		    // This parse failed, return overall read failure 
		    _ => {
			break next_form_parse;
		    }
		}

	    }
	}
    }
}
pub fn try_read_list(input: &[u8]/*changed*/) -> IResult<&[u8],Value> {
    named!(lparenp,
	   ws!(tag!("(")));
    named!(rparenp,
	   ws!(tag!(")")));
    
    let (list_inner_input,_) = lparenp(input)?;
    let mut list_as_vec = vec![];
    let mut rest_input = list_inner_input;
    loop {
	let right_paren = rparenp(rest_input);
	match right_paren {
	    Ok((after_list_input,_)) => {
		break Ok((after_list_input,list_as_vec.into_list().to_value()));
	    },
	    _ => {
		let next_form_parse = try_read(rest_input);
		match next_form_parse {
		    Ok((_rest_input,form)) => 	{
			list_as_vec.push(form.to_rc_value());
			rest_input = _rest_input;
		    },
		    // This parse failed, forward failure 
		    _ => {
			break next_form_parse;
		    }
		}

	    }
	}
    }
}
pub fn try_read(input: &[u8]/*changed*/) -> IResult<&[u8], Value> {
    
    preceded(multispace0,alt((try_read_string,try_read_symbol,try_read_i32,try_read_list,try_read_vector)))(input)
}
pub fn debug_try_read(input: &[u8]/*changed*/) -> IResult<&[u8], Value> {
    
    let reading = try_read(input);
    match &reading {
	Ok((_,value)) => println!("Reading: {}",value),
	_ => println!("Reading: {:?}",reading)
    };
    reading
}

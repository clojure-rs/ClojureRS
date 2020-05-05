use crate::value::{ToValue, Value};
use std::rc::Rc;
use crate::ifn::IFn;
use nom::lib::std::convert::TryFrom;

use std::fs::File;
use std::io::Read;

use url::Url;
use reqwest;

use std::error::Error;
use crate::error_message;

/// (slurp f & opts)
///
/// * Read a file provided (filename string) into a string (slurp "text.txt")
/// * GET an URL into a string (slurp "http://www.example.com")
/// TODO: clojure.java.io works with following types: Reader, BufferedReader,
/// TODO: InputStream, File, URI, URL, Socket, byte arrays, character arrays,
/// TODO and String
/// TODO local file URIs
///
#[derive(Debug, Clone)]
pub struct SlurpFn {}
impl ToValue for SlurpFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}

impl IFn for SlurpFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() >= 1 {
            let first_arg = &args.into_iter().next().unwrap().to_string();

            let possible_url = Url::parse(first_arg);

            match possible_url {
                Ok(url) => {
                    let rslt = reqwest::blocking::get(url.as_str());
                    match rslt {
                        Ok(res) => return Value::String(res.text().unwrap()),
                        Err(e) => return error_message::generic_err(Box::try_from(e).unwrap())
                    }
                },
                Err(e) => {
                    // try to find a file, if url parsing fails
                    let filename = first_arg;
                    let file_descriptor = File::open(filename);

                    let mut f = match file_descriptor {
                        Ok(file) => file,
                        Err(e) => {
                            return error_message::generic_err(Box::try_from(e).unwrap());
                        }
                    };
                    let mut s = String::new();
                    f.read_to_string(&mut s);
                    return Value::String(s)
                },
                _ => return Value::Nil
            };

        } else {
            Value::Nil
        }
    }
}
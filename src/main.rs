mod rust_core;
mod symbol;
mod type_tag;
mod value;

use rust_core::AddFn;
use crate::value::{IFn,ToValue};

fn main()
{
    let add_fn = AddFn{};
    let result = add_fn.invoke(&[&5_i32.to_value(),&6_i32.to_value(),&10_i32.to_value()]);

    println!("{:?}",result);
}




































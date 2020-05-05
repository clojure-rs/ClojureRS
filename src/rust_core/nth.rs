use crate::ifn::IFn;
use crate::value::{Value, ToValue};
use std::rc::Rc;
use crate::type_tag::TypeTag;

use crate::error_message;
use crate::persistent_list::PersistentList::Cons;
use crate::persistent_list::ToPersistentListIter;
use crate::persistent_vector::PersistentVector;

/// (nth coll index)
///
#[derive(Debug, Clone)]
pub struct NthFn {}
impl ToValue for NthFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for NthFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.len() != 2 {
            return error_message::wrong_varg_count(&[2,3], args.len())
        }
        // @TODO change iteration to work with Value references, or even change invoke to work on Rc<..>
        //       as we do everything else; surely we don't want to clone just to read from a collection
        if let Value::I32(ind) = **args.get(1).unwrap() {
            if ind < 0 {
                return error_message::index_cannot_be_negative(ind as usize)
            }
            let ind = ind as usize;

            match &**args.get(0).unwrap() {
                Value::PersistentList(Cons(head, tail, count)) => {
                    let count = *count as usize;
                    if ind >= count {
                        error_message::index_out_of_bounds(ind, count)
                    } else if ind == 0 {
                        head.to_value()
                    } else {
                        tail.iter().nth(ind - 1).unwrap().to_value()
                    }
                }
                Value::PersistentList(Empty) => error_message::index_out_of_bounds(ind, 0),
                Value::PersistentVector(PersistentVector { vals }) => {
                    if ind >= vals.len() {
                        error_message::index_out_of_bounds(ind, vals.len())
                    } else {
                        vals.get(ind).unwrap().to_value()
                    }
                }
                _ => error_message::type_mismatch(TypeTag::ISeq, &**args.get(0).unwrap()),
            }
        } else {
            error_message::type_mismatch(TypeTag::Integer, &**args.get(1).unwrap())
        }
    }
}
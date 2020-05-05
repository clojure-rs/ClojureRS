use crate::ifn::IFn;
use crate::value::{Value, ToValue};
use std::rc::Rc;

use crate::persistent_list::{ToPersistentListIter, PersistentList};
use crate::persistent_vector::ToPersistentVectorIter;


/// (concat x y & zs)
///
#[derive(Debug, Clone)]
pub struct ConcatFn {}
impl ToValue for ConcatFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ConcatFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        let concatted_vec = args.iter().fold(Vec::new(), |mut sum, coll| {
            let coll_vec = match &**coll {
                Value::PersistentList(plist) => {
                    Rc::new(plist.clone()).iter().collect::<Vec<Rc<Value>>>()
                }
                Value::PersistentVector(pvector) => {
                    Rc::new(pvector.clone()).iter().collect::<Vec<Rc<Value>>>()
                }
                _ => vec![],
            };

            sum.extend(coll_vec);
            sum
        });
        Value::PersistentList(concatted_vec.into_iter().collect::<PersistentList>())
    }
}
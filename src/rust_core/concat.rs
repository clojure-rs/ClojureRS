use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::persistent_list::{PersistentList, ToPersistentListIter};
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

#[cfg(test)]
mod tests {
    mod concat_tests {
        use crate::ifn::IFn;
        use crate::persistent_list::PersistentList;
        use crate::persistent_vector::PersistentVector;
        use crate::rust_core::ConcatFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn concat_test() {
            let concat = ConcatFn {};
            let s = "insert as first";
            let args = vec![
                Rc::new(Value::PersistentVector(
                    vec![
                        Rc::new(Value::String(String::from("1.1"))),
                        Rc::new(Value::String(String::from("1.2"))),
                    ]
                    .into_iter()
                    .collect::<PersistentVector>(),
                )),
                Rc::new(Value::PersistentVector(
                    vec![
                        Rc::new(Value::String(String::from("2.1"))),
                        Rc::new(Value::String(String::from("2.2"))),
                    ]
                    .into_iter()
                    .collect::<PersistentVector>(),
                )),
            ];
            assert_eq!(
                Value::PersistentList(
                    vec![
                        Rc::new(Value::String(String::from("1.1"))),
                        Rc::new(Value::String(String::from("1.2"))),
                        Rc::new(Value::String(String::from("2.1"))),
                        Rc::new(Value::String(String::from("2.2")))
                    ]
                    .into_iter()
                    .collect::<PersistentList>()
                ),
                concat.invoke(args)
            );
        }
    }
}

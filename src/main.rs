use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use dyn_clone::DynClone;

#[derive(Hash,PartialEq,Eq,Copy,Clone)]
struct Symbol {
    name: &'static str
}

trait IFn: DynClone {
    fn invoke(&self,args: &[&dyn Any]) -> Box<dyn Any>;
}
dyn_clone::clone_trait_object!(IFn);

#[derive(Clone)]
struct AddFn {
}
impl IFn for AddFn {
    fn invoke(&self,args: &[&dyn Any]) -> Box<dyn Any>
    {
	Box::new(args.into_iter().fold(0,|a,b|  {
	    let _a = a as i64;
	    if let Some(_b) = b.downcast_ref::<i64>() {
		_a + *_b
	    }
	    else {
		_a
	    }
	    
	}))
    }
}
use crate::Expr::*;
#[derive(Clone)]
enum Expr {
    SymbolExpr(Symbol),
    FnExpr(Box<dyn IFn>),
    i32Expr(i32),
    nilExpr 
}
impl Expr {
    fn eval(&self,environment: &HashMap<Symbol,Expr>) -> Expr{
	match self {
	    SymbolExpr(sym) => match environment.get(&sym) {
		Some(expr) => expr.clone(),
		_ => nilExpr
	    },
	    _ => nilExpr 
	}
	    
    }
}

fn main() {
    let add = AddFn{};
    let answer = add.invoke(&[&(1 as i64) as &dyn Any ,&(2 as i64) as &dyn Any]);
    let answer_as_num = answer.downcast_ref::<i64>();// as Box<i64>;
    
    println!("Start:");
    println!("{:?}",answer_as_num);
    println!("Edn:");
}

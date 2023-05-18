use crate::processing::symbols::{Operator, TypeSymbol};

use self::boolean::BoolWrapper;

mod defaults;
pub use defaults::*;

pub mod boolean;

pub trait UninstantiatedType {
    fn instantiate(&self) -> Box<dyn Type>;

    fn get_type_symbol(&self) -> TypeSymbol;
}

pub trait Type {
    fn get_type_symbol(&self) -> TypeSymbol;
    
    fn operate(&self, rhs: Box<dyn Type>) -> Result<(), String>;
}



pub trait Operation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self, rhs: Option<TypeSymbol>) -> Option<TypeSymbol>;

    fn operate(&self, lhs: &LHS, rhs: Box<dyn Type>) -> Result<(), String>;
}

pub struct TypeFactory {
    uninstantiated_types: Vec<Box<dyn UninstantiatedType>>
}

impl TypeFactory {
    pub fn new() -> Self {
        Self { 
            uninstantiated_types: vec![
                Box::new(BoolWrapper{})
            ]
        }
    }
}
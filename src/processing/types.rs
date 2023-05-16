use crate::processing::symbols::{Operator, TypeSymbol};

mod boolean;
pub use boolean::BoolType;

pub trait UninstantiatedType {
    fn instantiate(&self) -> Box<dyn Type>;

    fn get_type_symbol(&self) -> TypeSymbol;
}

pub trait Type {
    fn get_type_symbol(&self) -> TypeSymbol;
}

pub trait Operation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self, rhs: Option<TypeSymbol>) -> Option<TypeSymbol>;

    fn operate(&self, lhs: &LHS, rhs: Box<dyn Type>) -> Result<(), String>;
}


use crate::{processing::symbols::{TypeSymbol, Operator}, default_instruction_impl, default_type_wrapper_struct_and_impl, default_type_struct, default_type_initialiser, default_get_type_symbol_impl, default_type_operate_impl};

use super::{Operation, Type};

default_type_wrapper_struct_and_impl!(BoolWrapper, BoolType);
default_type_struct!(BoolType);
default_type_initialiser!(BoolType, BoolAnd);

impl Type for BoolType {
    default_get_type_symbol_impl!(BoolType, TypeSymbol::Boolean);
    default_type_operate_impl!(BoolType);
}


pub struct BoolAnd {}

impl Operation<BoolType> for BoolAnd {
    fn get_symbol(&self) -> Operator {
        Operator::And
    }

    fn get_result_type(&self, rhs: Option<TypeSymbol>) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs? {
            TypeSymbol::Boolean => Some(TypeSymbol::Boolean),
            _ => None
        }
    }

    fn operate(&self, lhs: &BoolType, rhs: Box<dyn Type>) -> Result<(), String> {
        todo!()
    }
}
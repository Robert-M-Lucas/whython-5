use crate::address::Address;
use crate::errors::create_literal_not_impl_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::symbols::Literal;
use crate::util::warn;
use crate::{
    default_get_type_symbol_impl, default_type_initialiser, default_type_operate_impl,
    default_type_struct, default_type_wrapper_struct_and_impl,
    processing::symbols::{Operator, TypeSymbol},
};

use super::{Operation, Type};

default_type_wrapper_struct_and_impl!(BoolWrapper, BoolType);
default_type_struct!(BoolType);
default_type_initialiser!(BoolType, BoolAnd);

pub const BOOL_TRUE: u8 = 0xFF;
pub const BOOL_FALSE: u8 = 0x00;

impl Type for BoolType {
    default_get_type_symbol_impl!(BoolType, TypeSymbol::Boolean);

    fn allocate_variable(
        &mut self,
        stack: &mut StackSizes,
        program_memory: &mut MemoryManager,
        to_assign: Option<&Literal>,
    ) -> Result<(), String> {
        if self.address.is_some() {
            warn(
                format!(
                    "Allocating {} when it already has a memory address",
                    self.get_type_symbol()
                )
                .as_str(),
            )
        }
        self.address = Some(Address::StackDirect(stack.increment_stack_size(1)));

        if let Some(literal) = to_assign {
            let constant = self.get_constant(literal)?;
            CopyInstruction::new_alloc(
                program_memory,
                &constant,
                self.address.as_ref().unwrap(),
                1,
            );
        }
        // ? If no literal memory will be default initialised to 0x00 (false)

        Ok(())
    }

    fn get_constant(&self, literal: &Literal) -> Result<Address, String> {
        match literal {
            Literal::Bool(value) => {
                if *value {
                    Ok(Address::Immediate(vec![BOOL_TRUE]))
                } else {
                    Ok(Address::Immediate(vec![BOOL_FALSE]))
                }
            }
            Literal::Int(value) => {
                if *value == 0 {
                    Ok(Address::Immediate(vec![BOOL_FALSE]))
                } else {
                    Ok(Address::Immediate(vec![BOOL_TRUE]))
                }
            }
            other => create_literal_not_impl_error(other, self.get_type_symbol()),
        }
    }

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
            _ => None,
        }
    }

    fn operate(&self, _lhs: &BoolType, _rhs: Box<dyn Type>) -> Result<(), String> {
        todo!()
    }
}

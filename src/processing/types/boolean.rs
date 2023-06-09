use crate::address::Address;
use crate::errors::create_literal_not_impl_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::binary_and_8::BinaryAndInstruction;
use crate::processing::instructions::binary_not_7::BinaryNotInstruction;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::symbols::Literal;
use crate::processing::types::PrefixOperation;
use crate::util::warn;
use crate::{
    default_get_type_symbol_impl, default_type_initialiser, default_type_operate_impl,
    default_type_struct, default_type_wrapper_struct_and_impl,
    processing::symbols::{Operator, TypeSymbol},
};

use super::{Operation, Type};

default_type_wrapper_struct_and_impl!(BoolWrapper, BoolType, TypeSymbol::Boolean);
default_type_struct!(BoolType);
default_type_initialiser!(BoolType, (BoolAnd), (BoolNot));

pub const BOOL_TRUE: u8 = 0xFF;
pub const BOOL_FALSE: u8 = 0x00;

pub const BOOLEAN_SIZE: usize = 1;

impl Type for BoolType {
    default_get_type_symbol_impl!(BoolType, TypeSymbol::Boolean);

    fn allocate_variable(
        &mut self,
        stack: &mut StackSizes,
        _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        if self.address.is_some() {
            warn(
                format!(
                    "Allocating {:?} when it already has a memory address",
                    self.get_type_symbol()
                )
                .as_str(),
            )
        }
        self.address = Some(Address::StackDirect(
            stack.increment_stack_size(BOOLEAN_SIZE),
        ));

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

    fn runtime_copy_from(&self, _other: &Box<dyn Type>) -> Result<(), String> {
        todo!()
    }

    fn runtime_copy_from_literal(
        &self,
        literal: &Literal,
        program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        let constant = self.get_constant(literal)?;
        CopyInstruction::new_alloc(program_memory, &constant, self.address.as_ref().unwrap(), 1);

        Ok(())
    }

    default_type_operate_impl!(BoolType);

    fn get_address_and_length(&self) -> (&Address, usize) {
        (self.address.as_ref().unwrap(), BOOLEAN_SIZE)
    }
}

pub struct BoolAnd {}

impl Operation<BoolType> for BoolAnd {
    fn get_symbol(&self) -> Operator {
        Operator::And
    }

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol> {
        // let rhs = rhs?;
        match rhs {
            TypeSymbol::Boolean => Some(TypeSymbol::Boolean),
            _ => None,
        }
    }

    fn operate(
        &self,
        lhs: &BoolType,
        rhs: &Box<dyn Type>,
        destination: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert!(matches!(destination.get_type_symbol(), TypeSymbol::Boolean));
        assert!(matches!(rhs.get_type_symbol(), TypeSymbol::Boolean));

        let (address_from, length) = lhs.get_address_and_length();
        BinaryAndInstruction::new_alloc(
            program_memory,
            address_from,
            rhs.get_address_and_length().0,
            destination.get_address_and_length().0,
            length,
        );
        Ok(())
    }
}

pub struct BoolNot {}

impl PrefixOperation<BoolType> for BoolNot {
    fn get_symbol(&self) -> Operator {
        Operator::And
    }

    fn get_result_type(&self) -> Option<TypeSymbol> {
        Some(TypeSymbol::Boolean)
    }

    fn operate_prefix(
        &self,
        lhs: &BoolType,
        destination: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
        _stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        assert!(matches!(destination.get_type_symbol(), TypeSymbol::Boolean));

        let (address_from, length) = lhs.get_address_and_length();
        BinaryNotInstruction::new_alloc(
            program_memory,
            address_from,
            destination.get_address_and_length().0,
            length,
        );
        Ok(())
    }
}

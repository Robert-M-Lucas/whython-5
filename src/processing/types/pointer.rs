use std::fmt::Pointer;
use crate::address::Address;
use crate::errors::create_literal_not_impl_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::binary_and_8::BinaryAndInstruction;
use crate::processing::instructions::binary_not_7::BinaryNotInstruction;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::symbols::Literal;
use crate::processing::types::{PrefixOperation, Type};
use crate::util::{warn, USIZE_BYTES};
use crate::{
    default_get_type_symbol_impl, default_type_initialiser, default_type_operate_impl,
    default_type_struct, default_type_wrapper_struct_and_impl,
    processing::symbols::{Operator, TypeSymbol},
};

default_type_wrapper_struct_and_impl!(PointerWrapper, PointerType, TypeSymbol::Pointer);
default_type_struct!(PointerType);
default_type_initialiser!(PointerType, (), ());

impl PointerType {
    pub fn duplicate_known(&self) -> PointerType {
        let mut t = PointerType::new();
        t.address = self.address.as_ref().and_then(|a| Some(a.clone()));
        t
    }
}

impl Type for PointerType {
    default_get_type_symbol_impl!(PointerType, TypeSymbol::Pointer);

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
            stack.increment_stack_size(USIZE_BYTES),
        ));

        Ok(())
    }

    fn get_constant(&self, literal: &Literal) -> Result<Address, String> {
        match literal {
            Literal::Int(value) => {
                let ptr: Result<usize, _> = value.clone().try_into();
                if let Ok(ptr) = ptr {
                    Ok(Address::Immediate(Vec::from(
                        ptr.to_le_bytes(),
                    )))
                }
                else {
                    Err(format!("The value ({}) can't fit into a {} (the value must be greater than zero and fit within your platform pointer width [{} bytes])", *value, self.get_type_symbol(), USIZE_BYTES))
                }
                }
                ,
            other => create_literal_not_impl_error(other, self.get_type_symbol()),
        }
    }

    fn runtime_copy_from(
        &self,
        other: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
    ) -> Result<CopyInstruction, String> {
        match other.get_type_symbol() {
            TypeSymbol::Pointer => {
                Ok(CopyInstruction::new_alloc(
                    program_memory,
                    other.get_address_and_length().0,
                    self.address.as_ref().unwrap(),
                    USIZE_BYTES,
                ))
            }
            s => Err(format!(
                "Copy not implemented from type '{}' to '{}'",
                s,
                TypeSymbol::Pointer
            )),
        }
    }

    fn runtime_copy_from_literal(
        &self,
        literal: &Literal,
        program_memory: &mut MemoryManager,
    ) -> Result<CopyInstruction, String> {
        let constant = self.get_constant(literal)?;

        Ok(CopyInstruction::new_alloc(
            program_memory,
            &constant,
            self.address.as_ref().unwrap(),
            USIZE_BYTES,
        ))
    }

    default_type_operate_impl!(PointerType);

    fn get_address_and_length(&self) -> (&Address, usize) {
        (self.address.as_ref().unwrap(), USIZE_BYTES)
    }

    fn get_address_mut(&mut self) -> &mut Address {
        self.address.as_mut().unwrap()
    }

    fn duplicate(&self) -> Box<dyn Type> {
        Box::new(self.duplicate_known())
    }
}

use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, TypeSymbol};
use crate::processing::types::{Type, TypeTrait};
use std::mem::size_of;

pub struct FunctionType {
    parameters: Vec<Type>,
    return_type: Option<Type>,
    start_address: usize,
    jump_variable_address: usize,
}

impl FunctionType {
    pub(crate) fn create_empty(
        parameters: Vec<Type>,
        return_type: Option<Type>,
        start_address: usize,
        jump_variable_address: usize,
    ) -> Self {
        Self {
            parameters,
            return_type,
            start_address,
            jump_variable_address,
        }
    }
}

impl TypeTrait for FunctionType {
    fn assign_clone(
        &self,
        _super: &Type,
        _memory_managers: &mut MemoryManagers,
        _to_clone: &Type,
    ) -> Result<(), String> {
        Err(format!(
            "{} can't be assigned from other function",
            self.get_type()
        ))
    }

    fn static_assign_literal(
        &self,
        _super: &Type,
        _memory_managers: &mut MemoryManagers,
        _literal: &Literal,
    ) -> Result<(), String> {
        Err(format!(
            "{} can't be assigned from literal",
            self.get_type()
        ))
    }

    fn get_type(&self) -> TypeSymbol {
        TypeSymbol::Function
    }

    fn get_return_type(&self) -> Result<TypeSymbol, String> {
        match &self.return_type {
            None => Err("Function does not return a value".to_string()),
            Some(return_type) => Ok(return_type.get_type()),
        }
    }

    fn get_size(&self) -> usize {
        0
    }

    fn call(
        &self,
        memory_managers: &mut MemoryManagers,
        arguments: Vec<&Type>,
        destination: Option<&Type>,
    ) -> Result<(), String> {
        if arguments.len() != self.parameters.len() {
            return Err("Wrong number of arguments".to_string());
        }

        // Copy arguments over
        for (index, argument) in arguments.into_iter().enumerate() {
            self.parameters[index].assign_clone(memory_managers, argument)?;
        }

        // Set jump back address
        let static_jump_back_address = memory_managers.variable_memory.append(
            &(memory_managers.program_memory.get_position()
                + CopyInstruction::get_size()
                + 2
                + JumpInstruction::get_size()
                + 2)
            .to_le_bytes(),
        );

        // Copy return value out of function
        CopyInstruction::new_alloc(
            memory_managers,
            static_jump_back_address,
            self.jump_variable_address,
            size_of::<usize>(),
        );

        // Create instruction to jump to function
        JumpInstruction::new_alloc(memory_managers, self.start_address);

        // Return value
        if destination.is_some() {
            if self.return_type.is_none() {
                return Err("Function does not return a value".to_string());
            }
            destination
                .unwrap()
                .assign_clone(memory_managers, self.return_type.as_ref().unwrap())?;
        }

        Ok(())
    }

    fn clone(&self) -> Box<dyn TypeTrait> {
        panic!("Functions should not be cloned!");
    }
}

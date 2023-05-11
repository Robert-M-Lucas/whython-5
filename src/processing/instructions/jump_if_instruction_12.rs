use super::Instruction;
use crate::processing::instructions::INSTRUCTION_CODE_LENGTH;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::boolean::BOOLEAN_TRUE;
use crate::processing::types::{Type, TypeSymbol};
use crate::util::get_usize;
use std::mem::size_of;

pub struct JumpIfInstruction {
    address: usize,
}

pub const JUMP_IF_NOT_INSTRUCTION_CODE: u16 = 12;

impl JumpIfInstruction {
    pub fn new_alloc(
        memory_managers: &mut MemoryManagers,
        condition_boolean: &Type,
        dest: usize,
    ) -> Self {
        if condition_boolean.get_type() != TypeSymbol::Boolean {
            panic!("Jump If instruction can only be created with a boolean condition")
        }

        let mut instruction_memory = vec![];
        instruction_memory.extend(JUMP_IF_NOT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(condition_boolean.get_address().to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn set_destination(&self, memory_managers: &mut MemoryManagers, dest: usize) {
        memory_managers.program_memory.overwrite(
            self.address + INSTRUCTION_CODE_LENGTH + size_of::<usize>(),
            &dest.to_le_bytes(),
        )
    }

    pub fn get_code() -> u16 {
        JUMP_IF_NOT_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 2 // Condition, dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!(
            "JUMP IF [{}] goto [{}]",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let condition = get_usize(pointer, &memory_managers.program_memory.memory);
        if memory_managers.variable_memory.memory[condition] == BOOLEAN_TRUE {
            *pointer += size_of::<usize>();
            *pointer = get_usize(pointer, &memory_managers.program_memory.memory);
        } else {
            *pointer += size_of::<usize>() * 2;
        }
    }
}

impl Instruction for JumpIfInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

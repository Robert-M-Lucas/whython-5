use super::Instruction;
use crate::processing::instructions::INSTRUCTION_CODE_LENGTH;
use crate::processing::processor::MemoryManagers;
use crate::util::get_usize;
use std::mem::size_of;

pub struct JumpVariableInstruction {
    address: usize,
}

pub const JUMP_VARIABLE_INSTRUCTION_CODE: u16 = 4;

impl JumpVariableInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, dest_variable: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(JUMP_VARIABLE_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(dest_variable.to_le_bytes());

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
        JUMP_VARIABLE_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() // Variable
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!("JUMP to variable [{}]", get_usize(&0, data),)
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let destination_variable = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer = get_usize(
            &destination_variable,
            &memory_managers.variable_memory.memory,
        );
    }
}

impl Instruction for JumpVariableInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

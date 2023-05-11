use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::boolean::{BOOLEAN_FALSE, BOOLEAN_TRUE};
use crate::util::get_usize;
use std::mem::size_of;

pub struct InvertInstruction {
    address: usize,
}

pub const INVERT_INSTRUCTION_CODE: u16 = 1;

/// Inverts the given boolean address from 0x00 to 0xFF
impl InvertInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, to_flip: usize, dest: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(INVERT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(to_flip.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        INVERT_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 2 // To flip, dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!(
            "INVERT [{}] dest [{}]",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let variable = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let dest = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        if memory_managers.variable_memory.memory[variable] == BOOLEAN_TRUE {
            memory_managers.variable_memory.memory[dest] = BOOLEAN_FALSE;
        } else {
            memory_managers.variable_memory.memory[dest] = BOOLEAN_TRUE;
        }
    }
}

impl Instruction for InvertInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::boolean::{BOOLEAN_FALSE, BOOLEAN_TRUE};
use crate::util::get_usize;
use std::mem::size_of;

pub struct AndInstruction {
    address: usize,
}

pub const AND_INSTRUCTION_CODE: u16 = 6;

/// Applies and to LHS and RHS
impl AndInstruction {
    pub fn new_alloc(
        memory_managers: &mut MemoryManagers,
        lhs: usize,
        rhs: usize,
        dest: usize,
    ) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(AND_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(lhs.to_le_bytes());
        instruction_memory.extend(rhs.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        AND_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 3 // LHS, RHS, dest
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!(
            "AND [{}], [{}] dest [{}]",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
            get_usize(&(size_of::<usize>() * 2), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let lhs = memory_managers.variable_memory.memory
            [get_usize(pointer, &memory_managers.program_memory.memory)];
        *pointer += size_of::<usize>();
        let rhs = memory_managers.variable_memory.memory
            [get_usize(pointer, &memory_managers.program_memory.memory)];
        *pointer += size_of::<usize>();
        let dest = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        if lhs == BOOLEAN_TRUE && rhs == BOOLEAN_TRUE {
            memory_managers.variable_memory.memory[dest] = BOOLEAN_TRUE;
        } else {
            memory_managers.variable_memory.memory[dest] = BOOLEAN_FALSE;
        }
    }
}

impl Instruction for AndInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

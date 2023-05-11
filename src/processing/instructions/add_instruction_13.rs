use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::util::get_usize;
use std::mem::size_of;

pub struct AddInstruction {
    address: usize,
}

pub const ADD_INSTRUCTION_CODE: u16 = 13;

impl AddInstruction {
    pub fn new_alloc(
        memory_managers: &mut MemoryManagers,
        lhs: usize,
        rhs: usize,
        len: usize,
        dest: usize,
    ) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(ADD_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(lhs.to_le_bytes());
        instruction_memory.extend(rhs.to_le_bytes());
        instruction_memory.extend(len.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        ADD_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 4
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!(
            "ADD [{} to {}] (len: {}) dest [{}]",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
            get_usize(&(size_of::<usize>() * 2), data),
            get_usize(&(size_of::<usize>() * 3), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let lhs = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let rhs = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let dest = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        let mut overflow = false;

        for i in 0..len {
            let a = memory_managers.variable_memory.memory[lhs + i];
            let b = memory_managers.variable_memory.memory[rhs + i];

            let result = if !overflow {
                a.wrapping_add(b)
            } else {
                a.wrapping_add(b).wrapping_add(1)
            }; // Carry

            overflow = result < a || result < b;

            memory_managers.variable_memory.memory[dest + i] = result;
        }
    }
}

impl Instruction for AddInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

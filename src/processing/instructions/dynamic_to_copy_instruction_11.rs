use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::util::get_usize;
use std::mem::size_of;

pub struct DynamicToCopyInstruction {
    address: usize,
}

pub const DYNAMIC_TO_COPY_INSTRUCTION_CODE: u16 = 11;

impl DynamicToCopyInstruction {
    pub fn new_alloc(
        memory_managers: &mut MemoryManagers,
        to_location: usize,
        indexing_size: usize,
        to_pointer_location: usize,
        direct_from: usize,
        length: usize,
    ) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(DYNAMIC_TO_COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(to_location.to_le_bytes());
        instruction_memory.extend(indexing_size.to_le_bytes());
        instruction_memory.extend(to_pointer_location.to_le_bytes());
        instruction_memory.extend(direct_from.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        DYNAMIC_TO_COPY_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 5
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!(
            "DYNAMIC COPY [{}] dest [{}:{}:{}] (len:{})",
            get_usize(&(size_of::<usize>() * 3), data),
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
            get_usize(&(size_of::<usize>() * 2), data),
            get_usize(&(size_of::<usize>() * 4), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let to_location = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let indexing_size = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let to_pointer = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let direct_from = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let length = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        let actual_to = get_usize(&to_pointer, &memory_managers.variable_memory.memory);

        for i in 0..length {
            memory_managers.variable_memory.memory[to_location + (actual_to * indexing_size) + i] =
                memory_managers.variable_memory.memory[direct_from + i];
        }
    }
}

impl Instruction for DynamicToCopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

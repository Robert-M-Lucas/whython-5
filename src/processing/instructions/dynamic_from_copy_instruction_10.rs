use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::util::get_usize;
use std::mem::size_of;

pub struct DynamicFromCopyInstruction {
    address: usize,
}

pub const DYNAMIC_FROM_COPY_INSTRUCTION_CODE: u16 = 10;

impl DynamicFromCopyInstruction {
    pub fn new_alloc(
        memory_managers: &mut MemoryManagers,
        from_location: usize,
        indexing_size: usize,
        from_pointer_location: usize,
        direct_to: usize,
        length: usize,
    ) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(DYNAMIC_FROM_COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(from_location.to_le_bytes());
        instruction_memory.extend(indexing_size.to_le_bytes());
        instruction_memory.extend(from_pointer_location.to_le_bytes());
        instruction_memory.extend(direct_to.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        DYNAMIC_FROM_COPY_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 5
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!(
            "DYNAMIC COPY [{}:{}:{}] (len:{}) dest [{}]",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
            get_usize(&(size_of::<usize>() * 2), data),
            get_usize(&(size_of::<usize>() * 4), data),
            get_usize(&(size_of::<usize>() * 3), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let from_location = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let indexing_size = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let from_pointer = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let direct_to = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let length = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        let actual_from = get_usize(&from_pointer, &memory_managers.variable_memory.memory);

        for i in 0..length {
            memory_managers.variable_memory.memory[direct_to + i] = memory_managers
                .variable_memory
                .memory[from_location + (actual_from * indexing_size) + i];
        }
    }
}

impl Instruction for DynamicFromCopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

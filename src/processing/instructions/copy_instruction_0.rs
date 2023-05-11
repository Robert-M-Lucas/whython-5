use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::util::get_usize;
use std::mem::size_of;

pub struct CopyInstruction {
    address: usize,
}

pub const COPY_INSTRUCTION_CODE: u16 = 0;

impl CopyInstruction {
    pub fn new_alloc(
        memory_managers: &mut MemoryManagers,
        from: usize,
        to: usize,
        length: usize,
    ) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(from.to_le_bytes());
        instruction_memory.extend(to.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        COPY_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 3 // From, To,  Length
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!(
            "COPY [{}] (len:{}) dest [{}]",
            get_usize(&0, data),
            get_usize(&(size_of::<usize>() * 2), data),
            get_usize(&size_of::<usize>(), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let from = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let to = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        for i in 0..len {
            memory_managers.variable_memory.memory[to + i] =
                memory_managers.variable_memory.memory[from + i];
        }
    }
}

impl Instruction for CopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

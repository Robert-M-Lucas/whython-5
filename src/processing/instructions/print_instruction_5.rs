use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::types::Type;
use crate::util::get_usize;
use std::mem::size_of;

pub struct PrintInstruction {
    address: usize,
}

pub const PRINT_INSTRUCTION_CODE: u16 = 5;

impl PrintInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, to_print: &Type, length: usize) -> Self {
        if length == 0 {
            panic!("Print length must be at least 1");
        }

        let mut instruction_memory = vec![];
        instruction_memory.extend(PRINT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(to_print.get_address().to_le_bytes());
        instruction_memory.extend(to_print.get_size().to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());

        // println!("{}", to_print.get_size() * length);

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        PRINT_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 3 // Address, LEn
    }

    pub(crate) fn get_debug(data: &[u8]) -> String {
        format!(
            "PRINT [{}] (len:{},{})",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
            get_usize(&(size_of::<usize>() * 2), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let position = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let count = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        println!(
            "{:X?}",
            &memory_managers.variable_memory.memory[position..(position + (count * len))]
        );
    }
}

impl Instruction for PrintInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

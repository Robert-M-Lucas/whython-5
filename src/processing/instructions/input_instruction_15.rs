use super::Instruction;
use crate::processing::processor::MemoryManagers;
use crate::util::get_usize;
use std::io::stdin;
use std::mem::size_of;

pub struct InputInstruction {
    address: usize,
}

pub const INPUT_INSTRUCTION_CODE: u16 = 15;

impl InputInstruction {
    pub fn new_alloc(memory_managers: &mut MemoryManagers, size: usize, dest: usize) -> Self {
        let mut instruction_memory = vec![];
        instruction_memory.extend(INPUT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(size.to_le_bytes());
        instruction_memory.extend(dest.to_le_bytes());

        assert_eq!(instruction_memory.len() - 2, Self::get_size());

        let address = memory_managers.program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_code() -> u16 {
        INPUT_INSTRUCTION_CODE
    }

    pub fn get_size() -> usize {
        size_of::<usize>() * 2
    }

    pub fn get_debug(data: &[u8]) -> String {
        format!(
            "INPUT (len: {}) dest [{}]",
            get_usize(&0, data),
            get_usize(&size_of::<usize>(), data),
        )
    }

    pub fn execute(pointer: &mut usize, memory_managers: &mut MemoryManagers) {
        let len = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();
        let dest = get_usize(pointer, &memory_managers.program_memory.memory);
        *pointer += size_of::<usize>();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Terminal read error");

        for i in 0..len {
            if i < input.len() {
                memory_managers.variable_memory.memory[dest + i] =
                    input.chars().nth(i).unwrap() as u8;
            } else {
                memory_managers.variable_memory.memory[dest + i] = 0;
            }
        }
    }
}

impl Instruction for InputInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

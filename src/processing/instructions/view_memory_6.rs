use crate::address::Address;

use crate::memory::{MemoryLocation, MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::{
    Execute, Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH,
};
use crate::processing::types::Type;
use crate::util::get_usize;

pub struct ViewMemoryInstruction {
    address: usize,
}

pub const VIEW_MEMORY_INSTRUCTION_CODE: InstructionCodeType = 6;

impl ViewMemoryInstruction {
    pub fn new_alloc(program_memory: &mut MemoryManager, to_dump: &Box<dyn Type>) -> Self {
        let (address, length) = to_dump.get_address_and_length();

        #[allow(unused_mut)]
        let mut instruction_memory = Vec::with_capacity(Self::get_size() + INSTRUCTION_CODE_LENGTH);
        instruction_memory.extend(VIEW_MEMORY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(length.to_le_bytes());
        instruction_memory.extend(address.get_bytes());

        let address = program_memory.append(&instruction_memory);

        Self { address }
    }

    pub fn get_size() -> usize {
        0
    }

    #[allow(unused_variables)]
    pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
        let size = get_usize(pointer, program_memory);
        *pointer += Address::get_address_size(program_memory, *pointer, size);
        stringify!(PrintDumpInstruction).to_string()
    }
}

impl Instruction for ViewMemoryInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

impl Execute for ViewMemoryInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, _pointer: &mut usize) {
        let length = get_usize(_pointer, memory.program_memory());
        let data =
            Address::evaluate_address_to_data(_pointer, &MemoryLocation::Program, &length, memory);
        for i in data {
            print!("{:02X}", i);
        }
        println!();
    }
}

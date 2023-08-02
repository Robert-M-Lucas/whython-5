use crate::address::Address;

use crate::memory::{MemoryLocation, MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::{
    Execute, Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH,
};
use crate::processing::types::Type;
use crate::util::get_usize;

pub struct ViewMemoryDecInstruction {
    address: usize,
}

pub const VIEW_MEMORY_DEC_INSTRUCTION_CODE: InstructionCodeType = 16;

impl ViewMemoryDecInstruction {
    pub fn new_alloc(program_memory: &mut MemoryManager, to_dump: &Box<dyn Type>) -> Self {
        let (address, length) = (to_dump.get_address(), to_dump.get_length());

        #[allow(unused_mut)]
        let mut instruction_memory = Vec::with_capacity(Self::get_size() + INSTRUCTION_CODE_LENGTH);
        instruction_memory.extend(VIEW_MEMORY_DEC_INSTRUCTION_CODE.to_le_bytes());
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

impl Instruction for ViewMemoryDecInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

impl Execute for ViewMemoryDecInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, _pointer: &mut usize) {
        let length = get_usize(_pointer, memory.program_memory());
        let data =
            Address::evaluate_address_to_data(_pointer, &MemoryLocation::Program, &length, memory);

        if data.len() > 16 {
            print!("Data too big for decimal representation - ");
            for i in data {
                print!("{:02X}", i);
            }
            println!();
        } else {
            let len = data.len();
            let mut data_full = Vec::with_capacity(16);
            for i in 0..16 {
                if i < len {
                    data_full.push(data[i]);
                } else {
                    data_full.push(0);
                }
            }
            println!("{}", u128::from_le_bytes(data_full.try_into().unwrap()));
        }
    }
}

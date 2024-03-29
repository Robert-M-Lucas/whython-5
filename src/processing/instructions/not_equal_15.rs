use crate::address::Address;
use crate::memory::{MemoryLocation, RuntimeMemoryManager};
use crate::processing::instructions::{
    Execute, Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH,
};
use crate::processing::types::boolean::{BOOLEAN_SIZE, BOOL_FALSE, BOOL_TRUE};
use crate::util::get_usize;

pub struct NotEqualInstruction {
    address: usize,
}

pub const NOT_EQUAL_INSTRUCTION_CODE: InstructionCodeType = 15;

impl NotEqualInstruction {
    pub fn new_alloc(
        program_memory: &mut crate::memory::MemoryManager,
        address_from_lhs: &Address,
        address_from_rhs: &Address,
        address_to: &Address,
        size: usize,
    ) -> Self {
        if address_to.is_immediate() {
            panic!(
                "Attempted to create NotEqualInstruction that overwrites Immediate (program) memory!"
            );
        }

        let size_bytes = size.to_le_bytes();
        let mut from_lhs_bytes = address_from_lhs.get_bytes();
        let mut from_rhs_bytes = address_from_rhs.get_bytes();
        let mut to_bytes = address_to.get_bytes();

        let mut instruction_memory = Vec::with_capacity(
            INSTRUCTION_CODE_LENGTH + from_lhs_bytes.len() + to_bytes.len() + size_bytes.len(),
        );
        instruction_memory.extend(NOT_EQUAL_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(size_bytes.iter());
        instruction_memory.append(&mut from_lhs_bytes);
        instruction_memory.append(&mut from_rhs_bytes);
        instruction_memory.append(&mut to_bytes);

        let address = program_memory.append(&instruction_memory);

        Self { address }
    }

    #[allow(unused_variables)]
    pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
        let size = get_usize(pointer, program_memory);
        *pointer += Address::get_address_size(program_memory, *pointer, size);
        *pointer += Address::get_address_size(program_memory, *pointer, size);
        *pointer += Address::get_address_size(program_memory, *pointer, size);
        "NotEqualInstruction".to_string()
    }
}

impl Execute for NotEqualInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, pointer: &mut usize) {
        let size = get_usize(pointer, memory.program_memory());
        let data_lhs =
            Address::evaluate_address_to_data(pointer, &MemoryLocation::Program, &size, memory);
        let data_rhs =
            Address::evaluate_address_to_data(pointer, &MemoryLocation::Program, &size, memory);
        let data_destination =
            Address::evaluate_address(pointer, &MemoryLocation::Program, &BOOLEAN_SIZE, memory);

        for i in 0..size {
            if data_lhs[i] != data_rhs[i] {
                memory.overwrite_data(&data_destination.1, data_destination.0, &[BOOL_TRUE]);
                return;
            }
        }

        memory.overwrite_data(&data_destination.1, data_destination.0, &[BOOL_FALSE]);
    }
}

impl Instruction for NotEqualInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

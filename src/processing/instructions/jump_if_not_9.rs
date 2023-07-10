use crate::address::Address;
use crate::memory::{MemoryLocation, MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::{Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH, Execute};
use crate::processing::types::boolean::{BOOL_TRUE, BOOLEAN_SIZE};
use crate::util::get_usize;

pub struct JumpIfNotInstruction {
    address: usize,
}

pub const JUMP_IF_NOT_INSTRUCTION_CODE: InstructionCodeType = 9;

impl JumpIfNotInstruction {
    pub fn new_alloc(
        memory_manager: &mut MemoryManager,
        boolean_address: &Address,
        destination: usize,
    ) -> Self {
        let mut boolean_address_bytes = boolean_address.get_bytes();
        let destination_bytes = destination.to_le_bytes();
        
        let mut instruction_memory = Vec::with_capacity(
            INSTRUCTION_CODE_LENGTH + boolean_address_bytes.len() + destination_bytes.len(),
        );
        instruction_memory.extend(JUMP_IF_NOT_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(destination_bytes.iter());
        instruction_memory.append(&mut boolean_address_bytes);

        let address = memory_manager.append(&instruction_memory);

        Self { address }
    }

    pub fn set_destination(&self, new_destination: usize, memory_manager: &mut MemoryManager) {
        memory_manager.overwrite(self.address + INSTRUCTION_CODE_LENGTH, &new_destination.to_le_bytes());
    }

    #[allow(unused_variables)]
    pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
        let destination = get_usize(pointer, program_memory);
        *pointer += Address::get_address_size(program_memory, *pointer, BOOLEAN_SIZE);
        "JumpIfNotInstruction".to_string()
    }
}

impl Execute for JumpIfNotInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, pointer: &mut usize) {
        let destination = get_usize(pointer, memory.program_memory());

        let boolean =
            Address::evaluate_address_to_data(pointer, &MemoryLocation::Program, &BOOLEAN_SIZE, memory)[0] == BOOL_TRUE;

        if !boolean {
            *pointer = destination;
        }
    }
}

impl Instruction for JumpIfNotInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

use crate::default_instruction_impl;
use crate::memory::{MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::{Execute, INSTRUCTION_CODE_LENGTH};
use crate::util::get_usize;

pub struct JumpInstruction {
    address: usize,
}

default_instruction_impl!(
    JumpInstruction,
    JUMP_INSTRUCTION_CODE,
    10,
    (destination, usize)
);

impl JumpInstruction {
    pub fn set_destination(&self, new_destination: usize, memory_manager: &mut MemoryManager) {
        memory_manager.overwrite(self.address + INSTRUCTION_CODE_LENGTH, &new_destination.to_le_bytes());
    }
}

impl Execute for JumpInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, mut pointer: &mut usize) {
        *pointer = get_usize(pointer, memory.program_memory());
    }
}
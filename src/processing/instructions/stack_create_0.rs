use crate::default_instruction_impl;
use crate::memory::{MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::INSTRUCTION_CODE_LENGTH;
use crate::util::get_usize;

pub struct StackCreateInstruction {
    address: usize,
}

default_instruction_impl!(
    StackCreateInstruction,
    STACK_CREATE_INSTRUCTION_CODE,
    0,
    (size, usize),
    (return_address, usize)
);

impl StackCreateInstruction {
    pub fn get_stack_size_and_return_addr(
        pointer: &mut usize,
        memory: &RuntimeMemoryManager,
    ) -> (usize, usize) {
        (
            get_usize(pointer, memory.program_memory()),
            get_usize(pointer, memory.program_memory()),
        )
    }

    pub fn change_stack_size(&mut self, memory: &mut MemoryManager, new_size: usize) {
        memory.overwrite(self.address + INSTRUCTION_CODE_LENGTH, &new_size.to_le_bytes());
    }
}

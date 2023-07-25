use crate::default_instruction_impl;
use crate::memory::{MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::{Execute, INSTRUCTION_CODE_LENGTH};
use crate::util::get_usize;

pub struct StackCreateInstruction {
    address: usize,
}

default_instruction_impl!(
    StackCreateInstruction,
    STACK_CREATE_INSTRUCTION_CODE,
    0,
    (size, usize)
);

impl StackCreateInstruction {
    pub fn get_stack_size(pointer: &mut usize, memory: &RuntimeMemoryManager) -> usize {
        get_usize(pointer, memory.program_memory())
    }

    pub fn set_stack_size(&mut self, new_size: usize, memory: &mut MemoryManager) {
        memory.overwrite(
            self.address + INSTRUCTION_CODE_LENGTH,
            &new_size.to_le_bytes(),
        );
    }

    // pub fn set_return_address(&self, memory: &mut MemoryManager, new_address: usize) {
    //     memory.overwrite(
    //         self.address + INSTRUCTION_CODE_LENGTH + USIZE_BYTES,
    //         &new_address.to_le_bytes(),
    //     );
    // }
}

impl Execute for StackCreateInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, mut pointer: &mut usize) {
        let size = StackCreateInstruction::get_stack_size(&mut pointer, memory);
        memory.stack_memory().create_stack(size);
    }
}

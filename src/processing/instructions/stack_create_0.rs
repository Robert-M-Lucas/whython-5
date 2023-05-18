use std::mem::size_of;
use crate::default_instruction_impl;
use crate::memory::RuntimeMemoryManager;
use crate::util::get_usize;

pub struct StackCreateInstruction {
    address: usize
}

default_instruction_impl!(StackCreateInstruction,
    STACK_CREATE_INSTRUCTION_CODE, 0,
    (size, usize), (return_address, usize));


impl StackCreateInstruction {
    pub fn get_stack_size_and_return_addr(pointer: &mut usize, memory: &RuntimeMemoryManager) -> (usize, usize) {
        (get_usize(pointer, memory.program_memory()), get_usize(pointer, memory.program_memory()))
    }
}
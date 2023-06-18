use crate::default_instruction_impl;
use crate::memory::RuntimeMemoryManager;
use crate::processing::instructions::Execute;

pub struct StackUpInstruction {
    address: usize,
}

default_instruction_impl!(StackUpInstruction, STACK_UP_INSTRUCTION_CODE, 1);

impl Execute for StackUpInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, _pointer: &mut usize) {
        memory.stack_memory().stack_up();
    }
}
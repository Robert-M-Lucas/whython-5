use crate::default_instruction_impl;
use crate::memory::RuntimeMemoryManager;
use crate::processing::instructions::Execute;

pub struct StackDownInstruction {
    address: usize,
}

default_instruction_impl!(StackDownInstruction, STACK_DOWN_INSTRUCTION_CODE, 4);

impl Execute for StackDownInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, _pointer: &mut usize) {
        memory.stack_memory().stack_down_and_delete()
    }
}

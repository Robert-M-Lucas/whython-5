use crate::default_instruction_impl;
use crate::memory::RuntimeMemoryManager;
use crate::processing::instructions::Execute;

pub struct DumpInstruction {
    address: usize,
}

default_instruction_impl!(DumpInstruction, DUMP_INSTRUCTION_CODE, 5);

impl Execute for DumpInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, _pointer: &mut usize) {
        memory.dump_all();
    }
}

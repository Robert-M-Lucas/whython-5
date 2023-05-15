use crate::default_instruction_impl;
use crate::memory_manager::{MemoryManager, RuntimeMemoryManager};
use crate::util::get_usize;

pub struct StackUpInstruction {
    address: usize
}

default_instruction_impl!(StackUpInstruction,
    STACK_UP_INSTRUCTION_CODE, 1);

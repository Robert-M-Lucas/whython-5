use crate::default_instruction_impl;

pub struct StackDownInstruction {
    address: usize,
}

default_instruction_impl!(StackDownInstruction, STACK_DOWN_INSTRUCTION_CODE, 4);

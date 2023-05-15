use crate::default_instruction_impl;



pub struct StackUpInstruction {
    address: usize
}

default_instruction_impl!(StackUpInstruction,
    STACK_UP_INSTRUCTION_CODE, 1);

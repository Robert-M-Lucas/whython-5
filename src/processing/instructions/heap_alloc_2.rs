use crate::default_instruction_impl;

pub struct HeapAllocInstruction {
    address: usize,
}

default_instruction_impl!(
    HeapAllocInstruction,
    HEAP_ALLOC_INSTRUCTION_CODE,
    2,
    (size, usize),
    (pointer_address, usize)
);

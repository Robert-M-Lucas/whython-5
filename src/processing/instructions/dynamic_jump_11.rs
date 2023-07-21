use crate::address::Address;
use crate::default_instruction_impl;
use crate::memory::{MemoryLocation, MemoryManager, RuntimeMemoryManager};
use crate::processing::instructions::{
    Execute, Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH,
};
use crate::util::{get_usize, USIZE_BYTES};
use std::usize;

pub struct DynamicJumpInstruction {
    address: usize,
}

pub const DYNAMIC_JUMP_INSTRUCTION_CODE: InstructionCodeType = 11;
impl DynamicJumpInstruction {
    pub fn new_alloc(program_memory: &mut MemoryManager, destination: &Address) -> Self {
        #[allow(unused_mut)]
        let mut instruction_memory = Vec::new();
        instruction_memory.extend(DYNAMIC_JUMP_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.append(&mut destination.get_bytes());

        // assert_eq!(instruction_memory.len() - INSTRUCTION_CODE_LENGTH, Self::get_size());

        let address = program_memory.append(&instruction_memory);

        Self { address }
    }

    // pub fn get_size() -> usize {
    //     0 + std::mem::size_of::<usize>()
    // }

    // ! Impossible!, address has variable length
    // pub fn set_destination(&self, new_destination: usize, program_memory: &mut MemoryManager) {
    //     program_memory.overwrite(
    //         self.address + INSTRUCTION_CODE_LENGTH,
    //         &new_destination.to_le_bytes(),
    //     );
    // }

    #[allow(unused_variables)]
    pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
        *pointer += Address::get_address_size(program_memory, *pointer, USIZE_BYTES);
        stringify!(DynamicJumpInstruction).to_string()
    }
}

impl Instruction for DynamicJumpInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

impl Execute for DynamicJumpInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, pointer: &mut usize) {
        let dest = usize::from_le_bytes(
            Address::evaluate_address_to_data(
                pointer,
                &MemoryLocation::Program,
                &USIZE_BYTES,
                memory,
            )
            .try_into()
            .unwrap(),
        );
        *pointer = dest;
    }
}

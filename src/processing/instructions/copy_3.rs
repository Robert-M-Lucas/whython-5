use crate::address::Address;
use crate::memory::{MemoryLocation, RuntimeMemoryManager};
use crate::processing::instructions::{Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH, Execute};
use crate::util::get_usize;

pub struct CopyInstruction {
    address: usize,
}

pub const COPY_INSTRUCTION_CODE: InstructionCodeType = 3;

impl CopyInstruction {
    pub fn new_alloc(
        memory_manager: &mut crate::memory::MemoryManager,
        address_from: &Address,
        address_to: &Address,
        size: usize,
    ) -> Self {
        if address_to.is_immediate() {
            panic!(
                "Attempted to create CopyInstruction that overwrites Immediate (program) memory!"
            );
        }

        let size_bytes = size.to_le_bytes();
        let mut from_bytes = address_from.get_bytes();
        let mut to_bytes = address_to.get_bytes();
        let mut instruction_memory = Vec::with_capacity(
            INSTRUCTION_CODE_LENGTH + from_bytes.len() + to_bytes.len() + size_bytes.len(),
        );
        instruction_memory.extend(COPY_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(size_bytes.iter());
        instruction_memory.append(&mut from_bytes);
        instruction_memory.append(&mut to_bytes);

        let address = memory_manager.append(&instruction_memory);

        Self { address }
    }

    #[allow(unused_variables)]
    pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
        let size = get_usize(pointer, program_memory);
        *pointer += Address::get_address_size(program_memory, *pointer, size);
        *pointer += Address::get_address_size(program_memory, *pointer, size);
        "CopyInstruction".to_string()
    }
}

impl Execute for CopyInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, pointer: &mut usize) {
        let size = get_usize(pointer, memory.program_memory());
        let data_location =
            Address::evaluate_address(pointer, &MemoryLocation::Program, &size, memory);
        let data_destination =
            Address::evaluate_address(pointer, &MemoryLocation::Program, &size, memory);
        let data = Vec::from(memory.get_data(&data_location.1, data_location.0, size));
        memory.overwrite_data(&data_destination.1, data_destination.0, &data);
    }
}

impl Instruction for CopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

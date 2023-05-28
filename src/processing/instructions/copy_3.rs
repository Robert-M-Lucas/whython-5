use crate::address::Address;
use crate::processing::instructions::{Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH};
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
    pub fn get_debug(memory: &[u8], pointer: &mut usize) -> String {
        let size = get_usize(pointer, memory);
        *pointer += Address::get_address_size(memory, *pointer, size);
        *pointer += Address::get_address_size(memory, *pointer, size);
        stringify!($name).to_string()
    }
}

impl Instruction for CopyInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

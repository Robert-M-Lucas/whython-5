use crate::address::Address;
use crate::memory::{MemoryLocation, RuntimeMemoryManager};
use crate::processing::instructions::{Instruction, InstructionCodeType, INSTRUCTION_CODE_LENGTH, Execute};
use crate::util::{get_usize, USIZE_BYTES};

pub struct HeapAllocInstruction {
    address: usize,
}

pub const HEAP_ALLOC_INSTRUCTION_CODE: InstructionCodeType = 2;

impl HeapAllocInstruction {
    pub fn new_alloc(
        memory_manager: &mut crate::memory::MemoryManager,
        size: usize,
        write_frame_id_to: &Address
    ) -> Self {
        if write_frame_id_to.is_immediate() {
            panic!(
                "Can't write frame id to Immediate address!"
            );
        }

        let size_bytes = size.to_le_bytes();
        let mut output_bytes = write_frame_id_to.get_bytes();
        let mut instruction_memory = Vec::with_capacity(
            INSTRUCTION_CODE_LENGTH + size_bytes.len() + output_bytes.len(),
        );
        instruction_memory.extend(HEAP_ALLOC_INSTRUCTION_CODE.to_le_bytes());
        instruction_memory.extend(size_bytes.iter());
        instruction_memory.append(&mut output_bytes);

        let address = memory_manager.append(&instruction_memory);

        Self { address }
    }

    #[allow(unused_variables)]
    pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
        let _size = get_usize(pointer, program_memory);
        *pointer += Address::get_address_size(program_memory, *pointer, USIZE_BYTES);
        "HeapAllocInstruction".to_string()
    }
}

impl Execute for HeapAllocInstruction {
    fn execute(memory: &mut RuntimeMemoryManager, pointer: &mut usize) {
        let size = get_usize(pointer, memory.program_memory());
        let write_frame_id_to =
            Address::evaluate_address(pointer, &MemoryLocation::Program, &size, memory);

        let id = memory.heap_memory().create_frame(size);

        memory.overwrite_data(&write_frame_id_to.1, write_frame_id_to.0, &id.to_le_bytes())
    }
}

impl Instruction for HeapAllocInstruction {
    fn get_address(&self) -> usize {
        self.address
    }
}

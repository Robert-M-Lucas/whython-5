use crate::memory::MemoryManager;
use crate::processing::blocks::BlockHandler;
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_down_4::StackDownInstruction;
use crate::processing::instructions::stack_up_1::StackUpInstruction;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::Symbol;

pub struct BaseBlock {
    stack_create_instruction: Option<StackCreateInstruction>,
}

impl BaseBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        Box::new(Self {
            stack_create_instruction: None,
        })
    }
}

impl BlockHandler for BaseBlock {
    fn on_entry(
        &mut self,
        memory_manager: &mut MemoryManager,
        _reference_stack: &mut ReferenceStack,
        _symbol_line: &[Symbol],
    ) -> Result<(), String> {
        self.stack_create_instruction =
            Some(StackCreateInstruction::new_alloc(memory_manager, 0, 0));
        StackUpInstruction::new_alloc(memory_manager);
        Ok(())
    }

    fn on_exit(
        &mut self,
        memory_manager: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        _symbol_line: &[Symbol],
        current_stack_size: usize,
    ) -> Result<bool, String> {
        self.on_forced_exit(memory_manager, reference_stack, current_stack_size)?;
        Ok(true)
    }

    fn on_forced_exit(
        &mut self,
        memory_manager: &mut MemoryManager,
        _reference_stack: &mut ReferenceStack,
        current_stack_size: usize,
    ) -> Result<(), String> {
        self.stack_create_instruction
            .as_mut()
            .expect("No stack create instruction")
            .change_stack_size(memory_manager, current_stack_size);
        StackDownInstruction::new_alloc(memory_manager);
        Ok(())
    }
}

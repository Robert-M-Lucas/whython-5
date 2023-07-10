use crate::memory::MemoryManager;
use crate::processing::blocks::base_block::BaseBlock;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::blocks::if_block::IfBlock;

use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Block, Symbol};

pub struct IfLine {}

impl LineHandler for IfLine {
    fn process_line(
        line: &[Symbol],
        memory_manager: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(Block::If) => {
                match block_coordinator.add_block_handler(
                    IfBlock::new_block(),
                    memory_manager,
                    line,
                ) {
                    Err(e) => ProcessingResult::Failure(e),
                    Ok(_) => ProcessingResult::Success,
                }
            }
            _ => ProcessingResult::Unmatched,
        }
    }
}

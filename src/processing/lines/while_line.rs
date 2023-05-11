use crate::processing::blocks::BlockCoordinator;

use crate::processing::blocks::while_block::WhileBlock;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Block, Symbol};

pub struct WhileLine {}

impl LineHandler for WhileLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(Block::While) => {
                match block_coordinator.add_block_handler(
                    WhileBlock::new_block(),
                    memory_managers,
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

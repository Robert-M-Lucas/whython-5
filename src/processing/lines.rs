pub mod arithmetic;
pub mod base_block;
pub mod dump;
pub mod if_line;
pub mod printdump;
pub mod variable_assignment;
pub mod variable_initialisation;
pub mod while_line;
pub mod break_continue;

use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::Symbol;

pub trait LineHandler {
    /// Attempts to process a line
    ///
    /// # Returns
    /// * `ProcessingResult::Successful` if the line is matched
    /// * `ProcessingResult::Unmatched` if the line is unmatched
    /// * `ProcessingResult::Failure(reason)` if the line is matched but an error occurred while processing it
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult;
}

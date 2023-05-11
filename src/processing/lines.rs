use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;

pub mod arithmetic;
pub mod break_continue_line;
pub mod call_line;
pub mod function_line;
pub mod if_line;
pub mod indexed_variable_assignment_line;
pub mod input_line;
pub mod print_line;
pub mod variable_assignment_line;
pub mod variable_initialisation_line;
pub mod variable_initialisation_with_argument_line;
pub mod while_line;

pub trait LineHandler {
    /// Attempts to process a line
    ///
    /// # Returns
    /// * `ProcessingResult::Successful` if the line is matched
    /// * `ProcessingResult::Unmatched` if the line is unmatched
    /// * `ProcessingResult::Failure(reason)` if the line is matched but an error occurred while processing it
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult;
}

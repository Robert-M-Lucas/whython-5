use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;

pub struct CallLine {}

/// Standalone function calling
impl LineHandler for CallLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.len() != 2 {
            return ProcessingResult::Unmatched;
        }

        if !matches!(line[0], Symbol::Name(_)) {
            return ProcessingResult::Unmatched;
        }
        if !matches!(line[1], Symbol::ArithmeticBlock(_)) {
            return ProcessingResult::Unmatched;
        }

        match handle_arithmetic_section(
            memory_managers,
            block_coordinator.get_reference_stack(),
            line,
            None,
            false,
        ) {
            Err(e) => ProcessingResult::Failure(e),
            Ok(_) => ProcessingResult::Success,
        }
    }
}

use crate::processing::blocks::BlockCoordinator;
use crate::processing::instructions::input_instruction_15::InputInstruction;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Builtin, Symbol};

pub struct InputLine {}

impl LineHandler for InputLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.len() < 2 {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Builtin(Builtin::Input) => {}
            _ => return ProcessingResult::Unmatched,
        };

        let t = match &line[1] {
            Symbol::Name(name) => match block_coordinator.get_variable(name) {
                Err(e) => return ProcessingResult::Failure(e),
                Ok(t) => t,
            },
            _ => return ProcessingResult::Failure("Input must be followed by a Name".to_string()),
        };

        InputInstruction::new_alloc(memory_managers, t.get_size(), t.get_address());

        ProcessingResult::Success
    }
}

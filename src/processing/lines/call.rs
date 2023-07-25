use super::LineHandler;
use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::ProcessingResult;

use crate::processing::symbols::Symbol;

use crate::q;

pub struct CallLine {}

impl LineHandler for CallLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty()
            || !matches!(line[0], Symbol::Name(_))
            || !matches!(line[1], Symbol::List(_))
        {
            return ProcessingResult::Unmatched;
        }

        if line.len() > 2 {
            return ProcessingResult::Failure(
                "A call can't be followed by anything on the same line".to_string(),
            );
        }

        let name = match &line[0] {
            Symbol::Name(name) => name,
            _ => panic!(),
        };

        let args = match &line[1] {
            Symbol::List(args) => args,
            _ => panic!(),
        };

        let (mut function_reference, offset) = q!(block_coordinator
            .get_reference_stack_mut()
            .get_and_remove_reference(name.as_str()));

        let (stack_sizes, reference_stack) =
            block_coordinator.get_stack_sizes_and_reference_stack();
        q!(
            q!(function_reference.get_function_mut())
            .call(
                None,
                args,
                program_memory,
                reference_stack,
                stack_sizes
            )
        );

        q!(reference_stack.register_reference_with_offset(function_reference, offset));

        ProcessingResult::Success
    }
}

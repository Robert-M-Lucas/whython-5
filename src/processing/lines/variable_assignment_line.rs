use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;

pub struct VariableAssignmentLine {}

impl LineHandler for VariableAssignmentLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.len() < 3 {
            return ProcessingResult::Unmatched;
        }

        let name = match &line[0] {
            Symbol::Name(name) => name,
            _ => return ProcessingResult::Unmatched,
        };

        let object = match block_coordinator.get_variable(name) {
            Err(e) => return ProcessingResult::Failure(e),
            Ok(object) => object,
        };

        let assigner = match &line[1] {
            Symbol::Assigner(assigner) => assigner,
            _ => return ProcessingResult::Failure("Name must be followed by assigner".to_string()),
        };

        let mut rhs = Vec::new();
        line[2..].clone_into(&mut rhs);

        let to_evaluate = assigner.get_expanded_equivalent(line[0].clone(), rhs);

        if let Err(e) = handle_arithmetic_section(
            memory_managers,
            block_coordinator.get_reference_stack(),
            &to_evaluate,
            Some(object),
            true,
        ) {
            return ProcessingResult::Failure(e);
        };

        ProcessingResult::Success
    }
}

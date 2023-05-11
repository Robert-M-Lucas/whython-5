use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;
use crate::processing::types::get_type_from_literal;

pub struct IndexedVariableAssignmentLine {}

impl LineHandler for IndexedVariableAssignmentLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.len() < 4 {
            return ProcessingResult::Unmatched;
        }

        // Get variable
        let name = match &line[0] {
            Symbol::Name(name) => name,
            _ => return ProcessingResult::Unmatched,
        };

        // Get index
        #[allow(unused_assignments)]
        let mut type_holder = None;
        let index = match &line[1] {
            Symbol::Indexer(symbol) => match symbol.as_ref() {
                Symbol::Name(name) => match block_coordinator.get_variable(name) {
                    Err(e) => return ProcessingResult::Failure(e),
                    Ok(value) => value,
                },
                Symbol::Literal(literal) => match get_type_from_literal(literal, memory_managers) {
                    Err(e) => return ProcessingResult::Failure(e),
                    Ok(value) => {
                        if let Err(e) = value.static_assign_literal(memory_managers, literal) {
                            return ProcessingResult::Failure(e);
                        }
                        type_holder = Some(value);
                        type_holder.as_ref().unwrap()
                    }
                },
                _ => return ProcessingResult::Unmatched,
            },
            _ => return ProcessingResult::Unmatched,
        };

        // Get value to be assigned
        let object = match block_coordinator.get_variable(name) {
            Err(e) => return ProcessingResult::Failure(e),
            Ok(object) => object,
        };

        // Get assigner
        let assigner = match &line[2] {
            Symbol::Assigner(assigner) => assigner,
            _ => {
                return ProcessingResult::Failure(
                    "Name and indexer must be followed by assigner".to_string(),
                )
            }
        };

        // Get assignment value
        let mut rhs = Vec::new();
        line[3..].clone_into(&mut rhs);

        let to_evaluate = assigner.get_expanded_equivalent(line[0].clone(), rhs);

        let result = match handle_arithmetic_section(
            memory_managers,
            block_coordinator.get_reference_stack(),
            &to_evaluate,
            None,
            true,
        ) {
            Err(e) => return ProcessingResult::Failure(e),
            Ok(value) => value.unwrap(),
        };

        // Write to object
        if let Err(e) = object.set_indexed(memory_managers, index, &result) {
            return ProcessingResult::Failure(e);
        }

        ProcessingResult::Success
    }
}

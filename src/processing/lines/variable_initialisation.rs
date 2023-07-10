use super::LineHandler;
use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::arithmetic::evaluate_arithmetic_into_type;
use crate::processing::processor::ProcessingResult;
use crate::processing::reference_manager::NamedReference;

use crate::processing::symbols::{Assigner, Symbol};
use crate::processing::types::TypeFactory;
use crate::q;

pub struct VariableInitialisationLine {}

impl LineHandler for VariableInitialisationLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() || !matches!(line[0], Symbol::Type(_)) {
            return ProcessingResult::Unmatched;
        }

        if line.len() < 4 {
            return ProcessingResult::Failure(
                "Type must be followed by a Name, '=' and value to initialise a variable"
                    .to_string(),
            );
        }

        let name = match &line[1] {
            Symbol::Name(name) => name,
            _ => {
                return ProcessingResult::Failure(
                    "Type must be followed by a Name to initialise a variable".to_string(),
                )
            }
        };

        match &line[2] {
            Symbol::Assigner(Assigner::Setter) => {}
            _ => {
                return ProcessingResult::Failure(
                    "Type must be followed by a Name, '=' and value to initialise a variable"
                        .to_string(),
                )
            }
        };

        let mut object = match &line[0] {
            Symbol::Type(type_symbol) => q!(TypeFactory::get_unallocated_type(type_symbol)),
            _ => panic!(),
        };

        q!(object.allocate_variable(block_coordinator.get_stack_sizes(), program_memory,));

        let (stack_sizes, reference_stack) =
            block_coordinator.get_stack_sizes_and_reference_stack();

        q!(evaluate_arithmetic_into_type(
            &line[3..],
            &object,
            program_memory,
            reference_stack,
            stack_sizes,
        ));

        if let Err(e) = block_coordinator
            .get_reference_stack_mut()
            .register_reference(NamedReference::new_variable(name.clone(), object))
        {
            return ProcessingResult::Failure(e);
        };

        ProcessingResult::Success
    }
}

use crate::memory::MemoryManager;
use super::LineHandler;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::ProcessingResult;
use crate::processing::reference_manager::NamedReference;
use crate::processing::symbols::{Assigner, Symbol};
use crate::processing::types::TypeFactory;
use crate::q;

pub struct VariableInitialisationLine {}

impl LineHandler for VariableInitialisationLine {
    fn process_line(
        line: &[Symbol],
        memory_manager: &mut MemoryManager,
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

        let literal = match &line[3] {
            Symbol::Literal(l) => l,
            _ => {
                return ProcessingResult::Failure("Initialisation is currently only possible from literals".to_string())
            }
        };

        let mut object = match &line[0] {
            Symbol::Type(type_symbol) => q!(TypeFactory::new().get_unallocated_type(type_symbol)),
            _ => panic!(),
        };

        /*if let Err(e) = handle_arithmetic_section(
            memory_managers,
            block_coordinator.get_reference_stack(),
            &line[3..],
            Some(&object),
            true,
        ) {
            return ProcessingResult::Failure(e);
        };*/

        q!(object.allocate_variable(block_coordinator.get_stack_sizes(), memory_manager, Some(literal)));


        if let Err(e) = block_coordinator
            .get_reference_stack_mut()
            .register_reference(NamedReference::new_variable(name.clone(), object))
        {
            return ProcessingResult::Failure(e);
        };

        ProcessingResult::Success
    }
}

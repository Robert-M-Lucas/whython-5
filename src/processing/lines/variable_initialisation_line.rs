use super::LineHandler;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Assigner, Symbol};
use crate::processing::types::get_type;

pub struct VariableInitialisationLine {}

impl LineHandler for VariableInitialisationLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
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
            Symbol::Type(type_symbol) => match get_type(type_symbol, memory_managers) {
                Err(e) => return ProcessingResult::Failure(e),
                Ok(value) => value,
            },
            _ => panic!(),
        };

        if let Err(e) = handle_arithmetic_section(
            memory_managers,
            block_coordinator.get_reference_stack(),
            &line[3..],
            Some(&object),
            true,
        ) {
            return ProcessingResult::Failure(e);
        };

        object.set_name(name.clone());
        if let Err(e) = block_coordinator
            .get_reference_stack_mut()
            .register_variable(object, name.clone())
        {
            return ProcessingResult::Failure(e);
        };

        ProcessingResult::Success
    }
}

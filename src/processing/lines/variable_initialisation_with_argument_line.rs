use super::LineHandler;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::MemoryManagers;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::{Assigner, Symbol};
use crate::processing::types::get_type;

pub struct VariableInitialisationWithArgumentLine {}

impl LineHandler for VariableInitialisationWithArgumentLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty()
            || !matches!(line[0], Symbol::Type(_))
            || !matches!(line[1], Symbol::Indexer(_))
        {
            return ProcessingResult::Unmatched;
        }

        if line.len() < 5 {
            return ProcessingResult::Failure(
                "Type must be followed Indexer, Name, '=' and value to initialise a variable"
                    .to_string(),
            );
        }

        let indexer = match &line[1] {
            Symbol::Indexer(symbol) => match symbol.as_ref() {
                Symbol::Literal(literal) => literal.clone(),
                _ => return ProcessingResult::Failure("Indexer must contain Literal".to_string()),
            },
            _ => panic!(),
        };

        let name = match &line[2] {
            Symbol::Name(name) => name,
            _ => {
                return ProcessingResult::Failure(
                    "Type and initialiser must be followed by a Name to initialise a variable"
                        .to_string(),
                )
            }
        };

        match &line[3] {
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

        let value = match &line[4] {
            Symbol::Literal(literal) => literal.clone(),
            _ => {
                return ProcessingResult::Failure(
                    "Can only assign Literals to indexed objects must contain Literal".to_string(),
                )
            }
        };

        if let Err(e) = object.create_indexed(memory_managers, &indexer, &value) {
            return ProcessingResult::Failure(e);
        }

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

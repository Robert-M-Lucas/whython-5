use crate::processing::blocks::BlockCoordinator;
use crate::processing::instructions::print_chars_instruction_9::PrintCharsInstruction;
use crate::processing::instructions::print_instruction_5::PrintInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Builtin, Literal, Symbol, TypeSymbol};
use crate::processing::types::get_type;

pub struct PrintLine {}

impl LineHandler for PrintLine {
    fn process_line(
        line: &[Symbol],
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Builtin(builtin) => {
                match builtin {
                    Builtin::Print => {
                        if line.len() == 1 {
                            return ProcessingResult::Failure(
                                "'print' must be followed by something to print".to_string(),
                            );
                        }
                        match handle_arithmetic_section(
                            memory_managers,
                            block_coordinator.get_reference_stack(),
                            &line[1..],
                            None,
                            true,
                        ) {
                            Err(e) => ProcessingResult::Failure(e),
                            Ok(value) => {
                                PrintInstruction::new_alloc(
                                    memory_managers,
                                    value.as_ref().unwrap(),
                                    value.as_ref().unwrap().get_len(),
                                );
                                ProcessingResult::Success
                            }
                        }
                    }
                    Builtin::PrintChars => {
                        if line.len() == 1 {
                            return ProcessingResult::Failure(
                                "'printc' must be followed by something to print".to_string(),
                            );
                        }
                        //? Special case for string literals to print correctly
                        if line.len() == 2 && matches!(line[1], Symbol::Literal(_)) {
                            if let Symbol::Literal(literal) = &line[1] {
                                if let Literal::String(string) = literal {
                                    let mut t =
                                        get_type(&TypeSymbol::Character, memory_managers).unwrap();
                                    t.create_indexed(
                                        memory_managers,
                                        &Literal::Int(string.len() as i64),
                                        literal,
                                    )
                                    .unwrap();

                                    PrintCharsInstruction::new_alloc(
                                        memory_managers,
                                        &t,
                                        t.get_len(),
                                    );
                                    return ProcessingResult::Success;
                                }
                            }
                        }
                        if line.len() == 2 && matches!(line[1], Symbol::Name(_)) {
                            match &line[1] {
                                Symbol::Name(name) => {
                                    let obj = match block_coordinator.get_variable(name) {
                                        Err(e) => return ProcessingResult::Failure(e),
                                        Ok(value) => value,
                                    };

                                    PrintCharsInstruction::new_alloc(
                                        memory_managers,
                                        obj,
                                        obj.get_len(),
                                    );
                                    ProcessingResult::Success
                                }
                                _ => panic!(),
                            }
                        } else {
                            match handle_arithmetic_section(
                                memory_managers,
                                block_coordinator.get_reference_stack(),
                                &line[1..],
                                None,
                                true,
                            ) {
                                Err(e) => ProcessingResult::Failure(e),
                                Ok(value) => {
                                    PrintCharsInstruction::new_alloc(
                                        memory_managers,
                                        value.as_ref().unwrap(),
                                        value.as_ref().unwrap().get_len(),
                                    );
                                    ProcessingResult::Success
                                }
                            }
                        }
                    }
                    _ => ProcessingResult::Unmatched,
                }
            }
            _ => ProcessingResult::Unmatched,
        }
    }
}

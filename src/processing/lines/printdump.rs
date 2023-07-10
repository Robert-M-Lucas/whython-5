use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::q;

use crate::processing::instructions::print_dump_6::PrintDumpInstruction;
use crate::processing::lines::LineHandler;
use crate::processing::processor::ProcessingResult;
use crate::processing::reference_manager::ReferenceType;
use crate::processing::symbols::{Keyword, Symbol};

pub struct PrintDumpLine {}

impl LineHandler for PrintDumpLine {
    fn process_line(
        line: &[Symbol],
        program_memory: &mut MemoryManager,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Keyword(Keyword::PrintDump) => {}
            _ => return ProcessingResult::Unmatched,
        };

        if line.len() != 2 {
            return ProcessingResult::Failure(
                "PrintDump must be followed by a variable".to_string(),
            );
        }

        let variable = match &line[1] {
            Symbol::Name(name) => {
                match &q!(block_coordinator.get_variable(name.as_str())).reference {
                    ReferenceType::Variable(variable) => variable,
                    _ => {
                        return ProcessingResult::Failure(
                            "PrintDump must be followed by a variable".to_string(),
                        )
                    }
                }
            }
            _ => {
                return ProcessingResult::Failure(
                    "PrintDump must be followed by a variable".to_string(),
                )
            }
        };

        PrintDumpInstruction::new_alloc(program_memory, variable);

        return ProcessingResult::Success;
    }
}

use crate::errors::create_line_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::base_block::BaseBlockLine;
use crate::processing::lines::break_continue::BreakContinueLine;
use crate::processing::lines::call::CallLine;
use crate::processing::lines::class_line::ClassLine;
use crate::processing::lines::dump::DumpLine;
use crate::processing::lines::function_line::FunctionLine;
use crate::processing::lines::if_line::IfLine;
use crate::processing::lines::variable_assignment::VariableAssignmentLine;
use crate::processing::lines::variable_initialisation::VariableInitialisationLine;
use crate::processing::lines::view_memory::ViewMemoryLine;
use crate::processing::lines::while_line::WhileLine;
use crate::processing::lines::LineHandler;
use crate::processing::preprocessor::SymbolData;
use crate::processing::symbols::Symbol;

pub enum ProcessingResult {
    Success,
    Unmatched,
    Failure(String),
}

impl ProcessingResult {
    /// Calls `f` if state is `Unmatched` returns `self` otherwise
    pub fn or_else<F: FnOnce() -> ProcessingResult>(self, f: F) -> ProcessingResult {
        match self {
            Self::Success | Self::Failure(_) => self,
            Self::Unmatched => f(),
        }
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure(_))
    }
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
    pub fn is_unmatched(&self) -> bool {
        matches!(self, Self::Unmatched)
    }

    pub fn get_error(self) -> String {
        match self {
            Self::Failure(e) => e,
            _ => panic!("Attempted to get error where there was none!"),
        }
    }
}

#[macro_export]
macro_rules! q {
    ($r: expr) => {
        match $r {
            Ok(v) => v,
            Err(e) => return $crate::processing::processor::ProcessingResult::Failure(e),
        }
    };
}

macro_rules! process_line {
    ($line: ident, $symbol_line: expr, $program_memory: expr, $block_coordinator: expr) => {
        $line::process_line(&$symbol_line, &mut $program_memory, &mut $block_coordinator)
    };
}

/// Takes symbol lines as an input and outputs compiled memory
pub fn process_symbols(symbol_data: SymbolData) -> Result<MemoryManager, String> {
    let mut memory = MemoryManager::new();

    let mut block_coordinator = BlockCoordinator::new(&mut memory);

    let line_count = symbol_data.lines.len();

    'line_iterator: for (line_index, line) in symbol_data.lines.iter().enumerate() {
        //? Skip empty lines
        if line.symbols.is_empty() {
            continue;
        }

        let indentation = line.indentation;
        let symbol_line = &line.symbols;

        //? Error if indentation is skipped
        if indentation > block_coordinator.get_indentation() {
            return create_line_error("Indentation to high".to_string(), line_index, &symbol_data);
        }

        //? Exit blocks until block indentation matches code indentation
        while block_coordinator.get_indentation() >= 1
            && indentation < block_coordinator.get_indentation()
        {
            if block_coordinator.get_indentation() >= 2
                && indentation <= block_coordinator.get_indentation() - 2
            {
                let result = block_coordinator.force_exit_block_handler(&mut memory);
                if let Err(e) = result {
                    return create_line_error(e, line_index, &symbol_data);
                }
            } else {
                let result = block_coordinator.exit_block_handler(&mut memory, symbol_line);
                if let Err(e) = result {
                    return create_line_error(e, line_count, &symbol_data);
                }
                if !result.unwrap() {
                    continue 'line_iterator;
                }
            }
        }

        //? Process line
        // let r = ProcessingResult::Failure("".to_string());
        let r = process_line!(BaseBlockLine, symbol_line, memory, block_coordinator)
            .or_else(|| {
                process_line!(
                    VariableInitialisationLine,
                    symbol_line,
                    memory,
                    block_coordinator
                )
            })
            .or_else(|| process_line!(DumpLine, symbol_line, memory, block_coordinator))
            .or_else(|| process_line!(ViewMemoryLine, symbol_line, memory, block_coordinator))
            .or_else(|| {
                process_line!(
                    VariableAssignmentLine,
                    symbol_line,
                    memory,
                    block_coordinator
                )
            })
            .or_else(|| process_line!(IfLine, symbol_line, memory, block_coordinator))
            .or_else(|| process_line!(WhileLine, symbol_line, memory, block_coordinator))
            .or_else(|| process_line!(BreakContinueLine, symbol_line, memory, block_coordinator))
            .or_else(|| process_line!(FunctionLine, symbol_line, memory, block_coordinator))
            .or_else(|| process_line!(CallLine, symbol_line, memory, block_coordinator))
            .or_else(|| process_line!(ClassLine, symbol_line, memory, block_coordinator));

        //? Handle unmatched / failed line
        if r.is_failure() {
            return create_line_error(r.get_error(), line_index, &symbol_data);
        } else if r.is_unmatched() {
            return create_line_error(
                "Line didn't match any known patterns".to_string(),
                line_index,
                &symbol_data
            );
        }

        if let Err(e) = block_coordinator.on_line_processed() {
            return create_line_error(
                e,
                line_index,
                &symbol_data
            );
        }
    }

    //? Exit remaining blocks
    while block_coordinator.get_indentation() >= 1 {
        let result = block_coordinator.force_exit_block_handler(&mut memory);
        if let Err(e) = result {
            return create_line_error(e, line_count, &symbol_data);
        }
    }

    block_coordinator.complete(&mut memory);

    Ok(memory)
}

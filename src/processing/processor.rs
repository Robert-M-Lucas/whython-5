use crate::errors::create_line_error;
use crate::memory::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::base_block::BaseBlockLine;
use crate::processing::lines::dump::DumpLine;
use crate::processing::lines::variable_initialisation::VariableInitialisationLine;
use crate::processing::lines::LineHandler;
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
            Err(e) => return crate::processing::processor::ProcessingResult::Failure(e),
        }
    };
}

macro_rules! process_line {
    ($line: ident, $symbol_line: expr, $memory_managers: expr, $block_coordinator: expr) => {
        $line::process_line(
            &$symbol_line,
            &mut $memory_managers,
            &mut $block_coordinator,
        )
    };
}

/// Takes symbol lines as an input and outputs compiled memory
pub fn process_symbols(symbols: Vec<(usize, Vec<Symbol>)>) -> Result<MemoryManager, String> {
    let mut memory = MemoryManager::new();

    let mut block_coordinator = BlockCoordinator::new(&mut memory);

    let line_count = symbols.len();

    'line_iterator: for (line_index, line) in symbols.into_iter().enumerate() {
        //? Skip empty lines
        if line.1.is_empty() {
            continue;
        }

        let indentation = line.0;
        let symbol_line = line.1;

        //? Error if indentation is skipped
        if indentation > block_coordinator.get_indentation() {
            return create_line_error("Indentation to high".to_string(), line_index);
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
                    return create_line_error(e, line_index);
                }
            } else {
                let result = block_coordinator.exit_block_handler(&mut memory, &symbol_line);
                if let Err(e) = result {
                    return create_line_error(e, line_count);
                }
                if !result.unwrap() {
                    continue 'line_iterator;
                }
            }
        }

        //? Process line
        // let r = ProcessingResult::Failure("".to_string());
        let r =
            process_line!(BaseBlockLine, symbol_line, memory, block_coordinator).or_else(|| {
                process_line!(
                    VariableInitialisationLine,
                    symbol_line,
                    memory,
                    block_coordinator
                )
            }).or_else(|| {
                process_line!(
                    DumpLine,
                    symbol_line,
                    memory,
                    block_coordinator
                )
            });

        //? Handle unmatched / failed line
        if r.is_failure() {
            return create_line_error(r.get_error(), line_index);
        } else if r.is_unmatched() {
            return create_line_error(
                "Line didn't match any known patterns".to_string(),
                line_index,
            );
        }
    }

    //? Exit remaining blocks
    while block_coordinator.get_indentation() >= 1 {
        let result = block_coordinator.force_exit_block_handler(&mut memory);
        if let Err(e) = result {
            return create_line_error(e, line_count);
        }
    }

    block_coordinator.complete(&mut memory);

    Ok(memory)
}

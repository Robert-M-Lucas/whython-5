use crate::errors::create_line_error;
use crate::memory_manager::MemoryManager;
use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::break_continue_line::BreakContinueLine;
use crate::processing::lines::call_line::CallLine;
use crate::processing::lines::function_line::FunctionLine;
use crate::processing::lines::if_line::IfLine;
use crate::processing::lines::indexed_variable_assignment_line::IndexedVariableAssignmentLine;
use crate::processing::lines::input_line::InputLine;
use crate::processing::lines::print_line::PrintLine;
use crate::processing::lines::variable_assignment_line::VariableAssignmentLine;
use crate::processing::lines::variable_initialisation_line::VariableInitialisationLine;
use crate::processing::lines::variable_initialisation_with_argument_line::VariableInitialisationWithArgumentLine;
use crate::processing::lines::while_line::WhileLine;
use crate::processing::lines::LineHandler;
use crate::processing::symbols::Symbol;
use crate::util::get_usize;
use num_format::{Locale, ToFormattedString};
use std::fs;
use std::io::Write;
use std::mem::size_of;

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

pub struct MemoryManagers {
    pub program_memory: MemoryManager,
    pub variable_memory: MemoryManager,
}

impl MemoryManagers {
    /// Saves memory data to an encoded file that can be loaded by `load_from_compiled`.
    /// # Save file format
    /// * Size of variable memory (`usize`)
    /// * Variable memory
    /// * Program memory
    pub fn save_to_compiled(&self, name: String) {
        let mut to_save = Vec::new();
        to_save.append(&mut Vec::from(
            self.variable_memory.get_position().to_le_bytes(),
        ));
        to_save.extend(&self.variable_memory.memory);
        to_save.extend(&self.program_memory.memory);

        let name = name + format!(" - {}.cwhy", (usize::BITS as usize)).as_str();

        println!(
            "Saving compiled data '{}' [{} bytes - {{{}:{}}}]",
            &name,
            to_save.len().to_formatted_string(&Locale::en),
            self.variable_memory
                .get_position()
                .to_formatted_string(&Locale::en),
            self.program_memory
                .get_position()
                .to_formatted_string(&Locale::en)
        );

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(name);

        if file.is_err() {
            println!("Failed to open file - {}", file.unwrap_err());
            return;
        }

        let mut file = file.unwrap();
        let r = file.write_all(&to_save);
        if r.is_err() {
            println!("Failed to write to file - {}", r.unwrap_err())
        }
    }

    /// Loads memory data from an encoded file that can be created by `save_from_compiled`.
    /// # Save file format
    /// * Size of variable memory (`usize`)
    /// * Variable memory
    /// * Program memory
    pub fn load_from_compiled(path: String) -> Result<Self, String> {
        println!("Loading precompiled data from file '{}'", &path);

        let data = match fs::read(path) {
            Err(e) => return Err(e.to_string()),
            Ok(value) => value,
        };

        let variable_memory_length = get_usize(&0, &data);
        let mut variable_memory = Vec::with_capacity(variable_memory_length);
        let mut program_memory =
            Vec::with_capacity(data.len() - variable_memory_length - size_of::<usize>());

        for i in data
            .iter()
            .skip(size_of::<usize>())
            .take(variable_memory_length)
        {
            variable_memory.push(*i);
        }

        for i in data
            .iter()
            .skip(size_of::<usize>() + variable_memory_length)
        {
            program_memory.push(*i);
        }

        Ok(Self {
            variable_memory: MemoryManager {
                memory: variable_memory,
            },
            program_memory: MemoryManager {
                memory: program_memory,
            },
        })
    }
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
pub fn process_symbols(symbols: Vec<(usize, Vec<Symbol>)>) -> Result<MemoryManagers, String> {
    let mut memory_managers = MemoryManagers {
        program_memory: MemoryManager::new(),
        variable_memory: MemoryManager::new(),
    };

    let mut block_coordinator = BlockCoordinator::new();

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
                let result = block_coordinator.force_exit_block_handler(&mut memory_managers);
                if let Err(e) = result {
                    return create_line_error(e, line_index);
                }
            } else {
                let result =
                    block_coordinator.exit_block_handler(&mut memory_managers, &symbol_line);
                if let Err(e) = result {
                    return create_line_error(e, line_count);
                }
                if !result.unwrap() {
                    continue 'line_iterator;
                }
            }
        }

        //? Process line
        let r = process_line!(
            VariableInitialisationWithArgumentLine,
            symbol_line,
            memory_managers,
            block_coordinator
        )
        .or_else(|| {
            process_line!(
                VariableInitialisationLine,
                symbol_line,
                memory_managers,
                block_coordinator
            )
        })
        .or_else(|| process_line!(CallLine, symbol_line, memory_managers, block_coordinator))
        .or_else(|| {
            process_line!(
                IndexedVariableAssignmentLine,
                symbol_line,
                memory_managers,
                block_coordinator
            )
        })
        .or_else(|| {
            process_line!(
                VariableAssignmentLine,
                symbol_line,
                memory_managers,
                block_coordinator
            )
        })
        .or_else(|| {
            process_line!(IfLine, symbol_line, memory_managers, block_coordinator).or_else(|| {
                process_line!(WhileLine, symbol_line, memory_managers, block_coordinator)
            })
        })
        .or_else(|| {
            process_line!(
                FunctionLine,
                symbol_line,
                memory_managers,
                block_coordinator
            )
        })
        .or_else(|| process_line!(PrintLine, symbol_line, memory_managers, block_coordinator))
        .or_else(|| process_line!(InputLine, symbol_line, memory_managers, block_coordinator))
        .or_else(|| {
            process_line!(
                BreakContinueLine,
                symbol_line,
                memory_managers,
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
        let result = block_coordinator.force_exit_block_handler(&mut memory_managers);
        if let Err(e) = result {
            return create_line_error(e, line_count);
        }
    }

    Ok(memory_managers)
}

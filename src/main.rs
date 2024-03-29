#![allow(dead_code)]

mod address;
mod errors;
mod execution;
pub mod file_loading;
mod memory;
mod processing;
mod translator;
pub mod util;

use crate::execution::execute;
use crate::memory::{MemoryManager, RuntimeMemoryManager};
use crate::processing::preprocessor::SymbolData;
#[allow(unused_imports)]
use crate::translator::translate;
use crate::util::{info, USIZE_BYTES};
use processing::preprocessor::convert_to_symbols;
use processing::processor::process_symbols;
use std::{env, fs};
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use crate::processing::symbols::TypeSymbol;

static CTRL_C: AtomicBool = AtomicBool::new(false);

const DEFAULT_FILE_NAME: &str = "main.why";

fn main() {
    ctrlc::set_handler(|| {
        CTRL_C.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    wrapped_main(&CTRL_C);

    #[cfg(not(debug_assertions))]
    util::pause();
}

//noinspection SpellCheckingInspection
fn wrapped_main(exit: &AtomicBool) {
    let args: Vec<String> = env::args().collect();
    info(
        format!(
            "Platform pointer (usize) length: {} [{}-bit]",
            USIZE_BYTES,
            USIZE_BYTES * 8
        )
        .as_str(),
    );

    let input_file = if args.len() >= 2 {
        args[1].clone()
    } else {
        DEFAULT_FILE_NAME.to_string()
    };

    let memory;

    let extension = match Path::new(&input_file).extension().and_then(OsStr::to_str) {
        None => {
            col_println!((red, bold), "Invalid input file '{}'", input_file);
            return;
        }
        Some(value) => value,
    };

    //? Compile
    if extension == "why" {
        println!("Starting compilation (stage 1)");
        let start = Instant::now();
        let mut symbol_data = SymbolData::new();
        match convert_to_symbols(input_file, &mut symbol_data) {
            Err(e) => {
                col_println!(
                    (red, bold),
                    "Compilation (stage 1) failed [{:?}]:\n\t{}",
                    start.elapsed(),
                    e
                );
                return;
            }
            Ok(value) => value,
        };

        col_println!(
            (green, bold),
            "Compilation (stage 1) completed [{:?}]",
            start.elapsed()
        );

        #[cfg(debug_assertions)]
        {
            let mut lexical_result = String::new();
            for l in &symbol_data.lines {
                for _ in 0..l.indentation {
                    lexical_result += "    ";
                }
                writeln!(lexical_result, "{:?}", l.symbols).unwrap();
            }
            fs::create_dir("dump").ok();
            let mut write = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("dump/lexical_result.txt")
                .expect("Unable to open file");

            write
                .write_all(lexical_result.as_str().as_ref())
                .expect("Failed to write to file");
        }

        println!("Starting compilation (stage 2)");
        let start = Instant::now();
        memory = match process_symbols(symbol_data) {
            Err(e) => {
                col_println!(
                    (red, bold),
                    "Compilation (stage 2) failed [{:?}]:\n    {}",
                    start.elapsed(),
                    e
                );
                return;
            }
            Ok(value) => value,
        };

        col_println!(
            (green, bold),
            "Compilation (stage 2) completed [{:?}]",
            start.elapsed()
        );

        let start = Instant::now();
        memory.save_to_file("Compiled".to_string());
        col_println!(
            (green, bold),
            "Saving compiled data completed [{:?}]",
            start.elapsed()
        );
    }
    //? Load compiled file
    else if extension == "cwhy" {
        let start = Instant::now();
        memory = match MemoryManager::load_from_file(input_file) {
            Err(e) => {
                col_println!((red, bold), "Loading precompiled file failed - {}", e);
                return;
            }
            Ok(value) => value,
        };
        col_println!(
            (green, bold),
            "Precompiled data loaded [{:?}]",
            start.elapsed()
        );
    } else {
        col_println!((red, bold), "Unrecognised extension '{}'", extension);
        return;
    }

    #[cfg(debug_assertions)]
    translate(&memory.memory, false);

    let mut runtime_memory = RuntimeMemoryManager::from_program_memory(memory);

    #[cfg(debug_assertions)]
    runtime_memory.dump_all("dump");

    if let Err(e) = execute(&mut runtime_memory, exit) {
        col_println!((red, bold), "Execution failed:\n\t{}", e)
    }

    #[cfg(debug_assertions)]
    runtime_memory.dump_all("dump/after-dump");
}

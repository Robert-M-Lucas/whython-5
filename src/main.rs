#![allow(dead_code)]

mod address;
mod errors;
mod execution;
mod memory;
mod processing;
mod translator;
pub mod util;

use crate::execution::execute;
use crate::memory::{MemoryManager, RuntimeMemoryManager};
#[allow(unused_imports)]
use crate::translator::translate;
use crate::util::{info, USIZE_BYTES};
use processing::preprocessor::convert_to_symbols;
use processing::processor::process_symbols;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

static CTRL_C: AtomicBool = AtomicBool::new(false);

fn main() {
    ctrlc::set_handler(|| {
        CTRL_C.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    wrapped_main(&CTRL_C);

    #[cfg(not(debug_assertions))]
    util::pause();
}

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
        "main.why".to_string()
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
        let input = match fs::read_to_string(&input_file) {
            Err(e) => {
                col_println!(
                    (red, bold),
                    "Error reading file '{}' - {}",
                    input_file,
                    e.to_string()
                );
                return;
            }
            Ok(value) => value,
        };

        println!("Starting compilation (pre)");
        let start = Instant::now();
        let r = match convert_to_symbols(input) {
            Err(e) => {
                col_println!(
                    (red, bold),
                    "Compilation (pre) failed [{:?}]:\n\t{}",
                    start.elapsed(),
                    e
                );
                return;
            }
            Ok(value) => value,
        };

        col_println!(
            (green, bold),
            "Compilation (pre) completed [{:?}]",
            start.elapsed()
        );

        println!("Starting compilation (post)");
        let start = Instant::now();
        memory = match process_symbols(r) {
            Err(e) => {
                col_println!(
                    (red, bold),
                    "Compilation (post) failed [{:?}]:\n\t{}",
                    start.elapsed(),
                    e
                );
                return;
            }
            Ok(value) => value,
        };

        col_println!(
            (green, bold),
            "Compilation (post) completed [{:?}]",
            start.elapsed()
        );

        memory.save_to_file("Compiled".to_string());
    }
    //? Load compiled file
    else if extension == "cwhy" {
        memory = match MemoryManager::load_from_file(input_file) {
            Err(e) => {
                col_println!((red, bold), "Loading precompiled file failed - {}", e);
                return;
            }
            Ok(value) => value,
        };
    } else {
        col_println!((red, bold), "Unrecognised extension '{}'", extension);
        return;
    }

    translate(&memory.memory, false);

    let mut runtime_memory = RuntimeMemoryManager::from_program_memory(memory);

    if let Err(e) = execute(&mut runtime_memory, exit) {
        col_println!((red, bold), "Execution failed:\n\t{}", e)
    }

    // runtime_memory.dump_all();
}

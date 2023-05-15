
use crate::col_println;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use crate::memory_manager::{RuntimeMemoryManager};
use crate::processing::instructions::stack_create_0::{STACK_CREATE_INSTRUCTION_CODE, StackCreateInstruction};

/// Executes the compiled program
pub fn execute(memory: &mut RuntimeMemoryManager, exit: &AtomicBool) -> Result<(), String> {
    let mut pointer: usize = 0;
    let program_length = memory.program_memory().len();

    println!("Executing program");
    let start_time = Instant::now();

    while pointer < program_length {
        let code = &memory.program_memory()[pointer..pointer + 2];
        pointer += 2;

        match u16::from_le_bytes(code.try_into().unwrap()) {
            STACK_CREATE_INSTRUCTION_CODE => {
                let (size, return_addr) = StackCreateInstruction::get_stack_size_and_return_addr(&mut pointer, memory);
                memory.stack_memory().create_stack(size, return_addr);
            },
            code => return Err(format!("Unknown code! [{}]", code)),
        };

        if exit.load(Ordering::Relaxed) {
            return Err("Program terminated by Ctrl+C".to_string());
        }
    }

    col_println!(
        (green, bold),
        "\nExecution completed [{:?}]",
        start_time.elapsed()
    );

    Ok(())
}

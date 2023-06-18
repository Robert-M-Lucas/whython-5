use crate::col_println;
use crate::memory::RuntimeMemoryManager;
use crate::processing::instructions::copy_3::{CopyInstruction, COPY_INSTRUCTION_CODE};
use crate::processing::instructions::stack_create_0::{
    StackCreateInstruction, STACK_CREATE_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_down_4::{STACK_DOWN_INSTRUCTION_CODE, StackDownInstruction};
use crate::processing::instructions::stack_up_1::{STACK_UP_INSTRUCTION_CODE, StackUpInstruction};
use crate::processing::instructions::InstructionCodeType;
use crate::util::warn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use crate::processing::instructions::dump_5::{DUMP_INSTRUCTION_CODE, DumpInstruction};
use crate::processing::instructions::Execute;


macro_rules! execute_instruction {
    ($instruction: ident, $memory: expr, $pointer: expr) => {
        $instruction::execute($memory, $pointer)
    };
}

/// Executes the compiled program
pub fn execute(memory: &mut RuntimeMemoryManager, exit: &AtomicBool) -> Result<(), String> {
    let mut pointer: usize = 0;
    let program_length = memory.program_memory().len();

    println!("Executing program");
    let start_time = Instant::now();

    while pointer < program_length {
        let code = &memory.program_memory()[pointer..pointer + 2];
        pointer += 2;

        match InstructionCodeType::from_le_bytes(code.try_into().unwrap()) {
            STACK_CREATE_INSTRUCTION_CODE => execute_instruction!(StackCreateInstruction, memory, &mut pointer),
            STACK_UP_INSTRUCTION_CODE => execute_instruction!(StackUpInstruction, memory, &mut pointer),
            STACK_DOWN_INSTRUCTION_CODE => execute_instruction!(StackDownInstruction, memory, &mut pointer),
            COPY_INSTRUCTION_CODE => execute_instruction!(CopyInstruction, memory, &mut pointer),
            DUMP_INSTRUCTION_CODE => execute_instruction!(DumpInstruction, memory, &mut pointer),
            code => return Err(format!("Unknown instruction code! [{}]", code)),
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

    if memory.stack_memory().get_current_level() != 0 {
        warn("Execution ended with a non-zero stack level")
    }

    Ok(())
}

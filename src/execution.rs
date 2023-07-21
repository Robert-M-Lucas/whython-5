use crate::col_println;
use crate::memory::RuntimeMemoryManager;
use crate::processing::instructions::binary_and_8::{
    BinaryAndInstruction, BINARY_AND_INSTRUCTION_CODE,
};
use crate::processing::instructions::binary_not_7::{
    BinaryNotInstruction, BINARY_NOT_INSTRUCTION_CODE,
};
use crate::processing::instructions::copy_3::{CopyInstruction, COPY_INSTRUCTION_CODE};
use crate::processing::instructions::dump_5::{DumpInstruction, DUMP_INSTRUCTION_CODE};
use crate::processing::instructions::dynamic_jump_11::{
    DynamicJumpInstruction, DYNAMIC_JUMP_INSTRUCTION_CODE,
};
use crate::processing::instructions::jump_if_not_9::{
    JumpIfNotInstruction, JUMP_IF_NOT_INSTRUCTION_CODE,
};
use crate::processing::instructions::jump_instruction_10::{
    JumpInstruction, JUMP_INSTRUCTION_CODE,
};
use crate::processing::instructions::print_dump_6::{
    PrintDumpInstruction, PRINT_DUMP_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_create_0::{
    StackCreateInstruction, STACK_CREATE_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_down_4::{
    StackDownInstruction, STACK_DOWN_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_up_1::{StackUpInstruction, STACK_UP_INSTRUCTION_CODE};
use crate::processing::instructions::Execute;
use crate::processing::instructions::InstructionCodeType;
use crate::util::warn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use crate::processing::instructions::binary_or_12::{BINARY_OR_INSTRUCTION_CODE, BinaryOrInstruction};

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
        let code = InstructionCodeType::from_le_bytes(
            (&memory.program_memory()[pointer..pointer + 2])
                .try_into()
                .unwrap(),
        );
        // println!("{} | {}", code, pointer);
        pointer += 2;

        match code {
            STACK_CREATE_INSTRUCTION_CODE => {
                execute_instruction!(StackCreateInstruction, memory, &mut pointer)
            }
            STACK_UP_INSTRUCTION_CODE => {
                execute_instruction!(StackUpInstruction, memory, &mut pointer)
            }
            STACK_DOWN_INSTRUCTION_CODE => {
                execute_instruction!(StackDownInstruction, memory, &mut pointer)
            }
            COPY_INSTRUCTION_CODE => execute_instruction!(CopyInstruction, memory, &mut pointer),
            DUMP_INSTRUCTION_CODE => execute_instruction!(DumpInstruction, memory, &mut pointer),
            PRINT_DUMP_INSTRUCTION_CODE => {
                execute_instruction!(PrintDumpInstruction, memory, &mut pointer)
            }
            BINARY_NOT_INSTRUCTION_CODE => {
                execute_instruction!(BinaryNotInstruction, memory, &mut pointer)
            }
            BINARY_AND_INSTRUCTION_CODE => {
                execute_instruction!(BinaryAndInstruction, memory, &mut pointer)
            }
            JUMP_IF_NOT_INSTRUCTION_CODE => {
                execute_instruction!(JumpIfNotInstruction, memory, &mut pointer)
            }
            JUMP_INSTRUCTION_CODE => execute_instruction!(JumpInstruction, memory, &mut pointer),
            DYNAMIC_JUMP_INSTRUCTION_CODE => {
                execute_instruction!(DynamicJumpInstruction, memory, &mut pointer)
            }
            BINARY_OR_INSTRUCTION_CODE => {
                execute_instruction!(BinaryOrInstruction, memory, &mut pointer)
            }
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

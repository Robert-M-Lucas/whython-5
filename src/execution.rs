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
use crate::processing::instructions::view_memory_6::{
    ViewMemoryInstruction, VIEW_MEMORY_INSTRUCTION_CODE,
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
use crate::processing::instructions::add_instruction_13::{ADD_INSTRUCTION_CODE, AddInstruction};
use crate::processing::instructions::binary_or_12::{BINARY_OR_INSTRUCTION_CODE, BinaryOrInstruction};
use crate::processing::instructions::equality_14::{EQUALITY_INSTRUCTION_CODE, EqualityInstruction};
use crate::processing::instructions::not_equal_15::{NOT_EQUAL_INSTRUCTION_CODE, NotEqualInstruction};
use crate::processing::instructions::view_memory_dec_16::{VIEW_MEMORY_DEC_INSTRUCTION_CODE, ViewMemoryDecInstruction};

macro_rules! execute {
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
            STACK_CREATE_INSTRUCTION_CODE => execute!(StackCreateInstruction, memory, &mut pointer),
            STACK_UP_INSTRUCTION_CODE => execute!(StackUpInstruction, memory, &mut pointer),
            STACK_DOWN_INSTRUCTION_CODE => execute!(StackDownInstruction, memory, &mut pointer),
            COPY_INSTRUCTION_CODE => execute!(CopyInstruction, memory, &mut pointer),
            DUMP_INSTRUCTION_CODE => execute!(DumpInstruction, memory, &mut pointer),
            VIEW_MEMORY_INSTRUCTION_CODE => execute!(ViewMemoryInstruction, memory, &mut pointer),
            BINARY_NOT_INSTRUCTION_CODE => execute!(BinaryNotInstruction, memory, &mut pointer),
            BINARY_AND_INSTRUCTION_CODE => execute!(BinaryAndInstruction, memory, &mut pointer),
            JUMP_IF_NOT_INSTRUCTION_CODE => execute!(JumpIfNotInstruction, memory, &mut pointer),
            JUMP_INSTRUCTION_CODE => execute!(JumpInstruction, memory, &mut pointer),
            DYNAMIC_JUMP_INSTRUCTION_CODE => execute!(DynamicJumpInstruction, memory, &mut pointer),
            BINARY_OR_INSTRUCTION_CODE => execute!(BinaryOrInstruction, memory, &mut pointer),
            ADD_INSTRUCTION_CODE => execute!(AddInstruction, memory, &mut pointer),
            EQUALITY_INSTRUCTION_CODE => execute!(EqualityInstruction, memory, &mut pointer),
            NOT_EQUAL_INSTRUCTION_CODE => execute!(NotEqualInstruction, memory, &mut pointer),
            VIEW_MEMORY_DEC_INSTRUCTION_CODE => execute!(ViewMemoryDecInstruction, memory, &mut pointer),
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

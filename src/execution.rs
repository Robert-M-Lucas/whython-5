use crate::col_println;
use crate::processing::instructions::add_instruction_13::AddInstruction;
use crate::processing::instructions::and_instruction_6::AndInstruction;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::dynamic_from_copy_instruction_10::DynamicFromCopyInstruction;
use crate::processing::instructions::dynamic_to_copy_instruction_11::DynamicToCopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::input_instruction_15::InputInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::jump_if_instruction_12::JumpIfInstruction;
use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::instructions::jump_variable_instruction_4::JumpVariableInstruction;
use crate::processing::instructions::not_equal_instruction_14::NotEqualInstruction;
use crate::processing::instructions::or_instruction_8::OrInstruction;
use crate::processing::instructions::print_chars_instruction_9::PrintCharsInstruction;
use crate::processing::instructions::print_instruction_5::PrintInstruction;
use crate::processing::processor::MemoryManagers;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Executes the compiled program
pub fn execute(memory_managers: &mut MemoryManagers, exit: &AtomicBool) -> Result<(), String> {
    let mut pointer: usize = 0;
    let program_length = memory_managers.program_memory.memory.len();

    println!("Executing program");
    let start_time = Instant::now();

    while pointer < program_length {
        let code = &memory_managers.program_memory.memory[pointer..pointer + 2];
        pointer += 2;

        match u16::from_le_bytes(code.try_into().unwrap()) {
            0 => CopyInstruction::execute(&mut pointer, memory_managers),
            1 => InvertInstruction::execute(&mut pointer, memory_managers),
            2 => JumpIfNotInstruction::execute(&mut pointer, memory_managers),
            3 => JumpInstruction::execute(&mut pointer, memory_managers),
            4 => JumpVariableInstruction::execute(&mut pointer, memory_managers),
            5 => PrintInstruction::execute(&mut pointer, memory_managers),
            6 => AndInstruction::execute(&mut pointer, memory_managers),
            7 => EqualInstruction::execute(&mut pointer, memory_managers),
            8 => OrInstruction::execute(&mut pointer, memory_managers),
            9 => PrintCharsInstruction::execute(&mut pointer, memory_managers),
            10 => DynamicFromCopyInstruction::execute(&mut pointer, memory_managers),
            11 => DynamicToCopyInstruction::execute(&mut pointer, memory_managers),
            12 => JumpIfInstruction::execute(&mut pointer, memory_managers),
            13 => AddInstruction::execute(&mut pointer, memory_managers),
            14 => NotEqualInstruction::execute(&mut pointer, memory_managers),
            15 => InputInstruction::execute(&mut pointer, memory_managers),
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

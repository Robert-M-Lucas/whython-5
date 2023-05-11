pub mod add_instruction_13;
pub mod and_instruction_6;
pub mod copy_instruction_0;
pub mod dynamic_from_copy_instruction_10;
pub mod dynamic_to_copy_instruction_11;
pub mod equal_instruction_7;
pub mod input_instruction_15;
pub mod invert_instruction_1;
pub mod jump_if_instruction_12;
pub mod jump_if_not_instruction_2;
pub mod jump_instruction_3;
pub mod jump_variable_instruction_4;
pub mod not_equal_instruction_14;
pub mod or_instruction_8;
pub mod print_chars_instruction_9;
pub mod print_instruction_5;

pub const INSTRUCTION_CODE_LENGTH: usize = 2;

pub trait Instruction {
    /// Returns the address of the instruction in program memory
    fn get_address(&self) -> usize;
}

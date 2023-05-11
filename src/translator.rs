use crate::processing::instructions::add_instruction_13::AddInstruction;
use crate::processing::instructions::and_instruction_6::AndInstruction;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::dynamic_from_copy_instruction_10::DynamicFromCopyInstruction;
use crate::processing::instructions::dynamic_to_copy_instruction_11::DynamicToCopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::input_instruction_15::InputInstruction;
use crate::processing::instructions::invert_instruction_1::InvertInstruction;
use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::instructions::jump_variable_instruction_4::JumpVariableInstruction;
use crate::processing::instructions::not_equal_instruction_14::NotEqualInstruction;
use crate::processing::instructions::or_instruction_8::OrInstruction;
use crate::processing::instructions::print_chars_instruction_9::PrintCharsInstruction;
use crate::processing::instructions::print_instruction_5::PrintInstruction;

macro_rules! translate {
    ($instruction: ident, $data: expr, $i: expr) => {
        (
            $instruction::get_debug(&$data[$i..$i + $instruction::get_size()]),
            $instruction::get_size(),
        )
    };
}

/// Prints the instructions and their data in the given memory
pub fn translate(data: &[u8], translate_one: bool) {
    println!("<------------------------------>");
    let mut i: usize = 0;
    while i < data.len() {
        print!("[{:0>5}] | ", i);

        let code = &data[i..i + 2];
        i += 2;
        let (output, size) = match u16::from_le_bytes(code.try_into().unwrap()) {
            0 => translate!(CopyInstruction, data, i),
            1 => translate!(InvertInstruction, data, i),
            2 => translate!(JumpIfNotInstruction, data, i),
            3 => translate!(JumpInstruction, data, i),
            4 => translate!(JumpVariableInstruction, data, i),
            5 => translate!(PrintInstruction, data, i),
            6 => translate!(AndInstruction, data, i),
            7 => translate!(EqualInstruction, data, i),
            8 => translate!(OrInstruction, data, i),
            9 => translate!(PrintCharsInstruction, data, i),
            10 => translate!(DynamicFromCopyInstruction, data, i),
            11 => translate!(DynamicToCopyInstruction, data, i),
            12 => translate!(JumpInstruction, data, i),
            13 => translate!(AddInstruction, data, i),
            14 => translate!(NotEqualInstruction, data, i),
            15 => translate!(InputInstruction, data, i),
            code => panic!("Debug not implemented for code {}", code),
        };

        println!("{}", output);

        if translate_one {
            break;
        }

        i += size;
    }
    println!("<------------------------------>");
}

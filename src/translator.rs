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
use crate::processing::instructions::heap_alloc_2::{
    HeapAllocInstruction, HEAP_ALLOC_INSTRUCTION_CODE,
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
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_create_0::STACK_CREATE_INSTRUCTION_CODE;
use crate::processing::instructions::stack_down_4::{
    StackDownInstruction, STACK_DOWN_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_up_1::{StackUpInstruction, STACK_UP_INSTRUCTION_CODE};
use crate::processing::instructions::{InstructionCodeType, INSTRUCTION_CODE_LENGTH};
use crate::processing::instructions::add_instruction_13::{ADD_INSTRUCTION_CODE, AddInstruction};
use crate::processing::instructions::binary_or_12::{BINARY_OR_INSTRUCTION_CODE, BinaryOrInstruction};
use crate::processing::instructions::equality_14::{EQUALITY_INSTRUCTION_CODE, EqualityInstruction};
use crate::processing::instructions::not_equal_15::{NOT_EQUAL_INSTRUCTION_CODE, NotEqualInstruction};
use crate::processing::instructions::view_memory_dec_16::{VIEW_MEMORY_DEC_INSTRUCTION_CODE, ViewMemoryDecInstruction};

macro_rules! translate {
    ($instruction: ident, $data: expr, $i: expr) => {
        $instruction::get_debug(&$data, &mut $i)
    };
}

/// Prints the instructions and their data in the given memory
pub fn translate(data: &[u8], translate_one: bool) {
    println!("<------------------------------>");
    let mut i: usize = 0;
    while i < data.len() {
        print!("[{:0>5}] | ", i);

        let code = &data[i..i + INSTRUCTION_CODE_LENGTH];
        i += INSTRUCTION_CODE_LENGTH;

        let output = match InstructionCodeType::from_le_bytes(code.try_into().unwrap()) {
            STACK_CREATE_INSTRUCTION_CODE => translate!(StackCreateInstruction, data, i),
            STACK_UP_INSTRUCTION_CODE => translate!(StackUpInstruction, data, i),
            HEAP_ALLOC_INSTRUCTION_CODE => translate!(HeapAllocInstruction, data, i),
            COPY_INSTRUCTION_CODE => translate!(CopyInstruction, data, i),
            STACK_DOWN_INSTRUCTION_CODE => translate!(StackDownInstruction, data, i),
            DUMP_INSTRUCTION_CODE => translate!(DumpInstruction, data, i),
            VIEW_MEMORY_INSTRUCTION_CODE => translate!(ViewMemoryInstruction, data, i),
            BINARY_NOT_INSTRUCTION_CODE => translate!(BinaryNotInstruction, data, i),
            BINARY_AND_INSTRUCTION_CODE => translate!(BinaryAndInstruction, data, i),
            JUMP_IF_NOT_INSTRUCTION_CODE => translate!(JumpIfNotInstruction, data, i),
            JUMP_INSTRUCTION_CODE => translate!(JumpInstruction, data, i),
            DYNAMIC_JUMP_INSTRUCTION_CODE => translate!(DynamicJumpInstruction, data, i),
            BINARY_OR_INSTRUCTION_CODE => translate!(BinaryOrInstruction, data, i),
            ADD_INSTRUCTION_CODE => translate!(AddInstruction, data, i),
            EQUALITY_INSTRUCTION_CODE => translate!(EqualityInstruction, data, i),
            NOT_EQUAL_INSTRUCTION_CODE => translate!(NotEqualInstruction, data, i),
            VIEW_MEMORY_DEC_INSTRUCTION_CODE => translate!(ViewMemoryDecInstruction, data, i),
            code =>  {
                println!("Debug not implemented for code {}. Terminating translation due to unknown instruction size.", code);
                return;
            },
        };

        println!("{}", output);

        if translate_one {
            break;
        }
    }
    println!("<------------------------------>");
}

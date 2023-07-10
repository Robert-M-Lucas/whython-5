use crate::processing::instructions::print_dump_6::{PRINT_DUMP_INSTRUCTION_CODE, PrintDumpInstruction};
use crate::processing::instructions::copy_3::{CopyInstruction, COPY_INSTRUCTION_CODE};
use crate::processing::instructions::dump_5::{DUMP_INSTRUCTION_CODE, DumpInstruction};
use crate::processing::instructions::heap_alloc_2::{
    HeapAllocInstruction, HEAP_ALLOC_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_create_0::STACK_CREATE_INSTRUCTION_CODE;
use crate::processing::instructions::stack_down_4::{
    StackDownInstruction, STACK_DOWN_INSTRUCTION_CODE,
};
use crate::processing::instructions::stack_up_1::{StackUpInstruction, STACK_UP_INSTRUCTION_CODE};
use crate::processing::instructions::{INSTRUCTION_CODE_LENGTH, InstructionCodeType};
use crate::processing::instructions::binary_and_8::{BINARY_AND_INSTRUCTION_CODE, BinaryAndInstruction};
use crate::processing::instructions::binary_not_7::{BINARY_NOT_INSTRUCTION_CODE, BinaryNotInstruction};

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
            PRINT_DUMP_INSTRUCTION_CODE => translate!(PrintDumpInstruction, data, i),
            BINARY_NOT_INSTRUCTION_CODE => translate!(BinaryNotInstruction, data, i),
            BINARY_AND_INSTRUCTION_CODE => translate!(BinaryAndInstruction, data, i),
            code => panic!("Debug not implemented for code {}", code),
        };

        println!("{}", output);

        if translate_one {
            break;
        }
    }
    println!("<------------------------------>");
}

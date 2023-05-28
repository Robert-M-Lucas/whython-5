use crate::processing::instructions::copy_3::{COPY_INSTRUCTION_CODE, CopyInstruction};
use crate::processing::instructions::heap_alloc_2::{HEAP_ALLOC_INSTRUCTION_CODE, HeapAllocInstruction};
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_create_0::STACK_CREATE_INSTRUCTION_CODE;
use crate::processing::instructions::stack_up_1::{STACK_UP_INSTRUCTION_CODE, StackUpInstruction};

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

        let code = &data[i..i + 2];
        i += 2;



        let output = match u16::from_le_bytes(code.try_into().unwrap()) {
            STACK_CREATE_INSTRUCTION_CODE => translate!(StackCreateInstruction, data, i),
            STACK_UP_INSTRUCTION_CODE => translate!(StackUpInstruction, data, i),
            HEAP_ALLOC_INSTRUCTION_CODE => translate!(HeapAllocInstruction, data, i),
            COPY_INSTRUCTION_CODE => translate!(CopyInstruction, data, i),
            code => panic!("Debug not implemented for code {}", code),
        };

        println!("{}", output);

        if translate_one {
            break;
        }
    }
    println!("<------------------------------>");
}

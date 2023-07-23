use crate::memory::RuntimeMemoryManager;

pub mod binary_and_8;
pub mod binary_not_7;
pub mod copy_3;
pub mod dump_5;
pub mod dynamic_jump_11;
pub mod heap_alloc_2;
pub mod jump_if_not_9;
pub mod jump_instruction_10;
pub mod print_dump_6;
pub mod stack_create_0;
pub mod stack_down_4;
pub mod stack_up_1;
pub mod binary_or_12;
pub mod add_instruction_13;

pub type InstructionCodeType = u16;
pub const INSTRUCTION_CODE_LENGTH: usize = 2;

#[macro_export]
macro_rules! default_instruction_impl {
    ($name: ident, $caps_name: ident, $code: expr $(, ($arg:ident, $t:ty) )*) => {
        pub const $caps_name: $crate::processing::instructions::InstructionCodeType = $code;

        impl $name {
            pub fn new_alloc(program_memory: &mut $crate::memory::MemoryManager, $($arg: $t),*) -> Self {
                #[allow(unused_mut)]
                let mut instruction_memory = Vec::with_capacity(Self::get_size() + $crate::processing::instructions::INSTRUCTION_CODE_LENGTH);
                instruction_memory.extend($caps_name.to_le_bytes());
                $(instruction_memory.extend($arg.to_le_bytes());
                )*

                assert_eq!(instruction_memory.len() - $crate::processing::instructions::INSTRUCTION_CODE_LENGTH, Self::get_size());

                let address = program_memory.append(&instruction_memory);

                Self { address }
            }

            pub fn get_size() -> usize {
                0 $(+ std::mem::size_of::<$t>())*
            }

            #[allow(unused_variables)]
            pub fn get_debug(program_memory: &[u8], pointer: &mut usize) -> String {
                *pointer += Self::get_size();
                stringify!($name).to_string()
            }
        }

        impl $crate::processing::instructions::Instruction for $name {
            fn get_address(&self) -> usize {
                self.address
            }
        }
    };
}

pub trait Instruction {
    /// Returns the address of the instruction in program memory
    fn get_address(&self) -> usize;
}

pub trait Execute {
    fn execute(memory: &mut RuntimeMemoryManager, pointer: &mut usize);
}

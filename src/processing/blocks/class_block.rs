use crate::bx;
use crate::memory::MemoryManager;
use crate::processing::blocks::{BlockHandler, StackSizes};
use crate::processing::reference_manager::class::ClassReference;
use crate::processing::reference_manager::{Reference, ReferenceStack};
use crate::processing::symbols::{Block, Symbol, CLASS_SELF_NAME};

pub struct ClassBlock {
    name: Option<String>,
}

impl ClassBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        bx!(Self { name: None })
    }
}

impl BlockHandler for ClassBlock {
    fn on_entry(
        &mut self,
        _program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        _stack_sizes: &mut StackSizes,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        if symbol_line.len() != 2 {
            return Err(format!(
                "Class declaration must be formatted {} [Name]",
                Block::Class.get_code_representation()
            ));
        }

        let name = match &symbol_line[1] {
            Symbol::Name(name) => {
                if name.len() != 1 {
                    return Err("Class names cannot have separators".to_string());
                }
                name[0].clone()
            }
            _ => {
                return Err(format!(
                    "Class declaration must be formatted {} [Name]",
                    Block::Class.get_code_representation()
                ))
            }
        };

        self.name = Some(name);

        reference_stack
            .register_reference_with_offset(
                Reference::Class(ClassReference::new()),
                vec![CLASS_SELF_NAME.to_string()],
                1,
            )
            .unwrap();

        Ok(())
    }

    fn on_forced_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String> {
        reference_stack
            .get_reference_handler_mut(&[CLASS_SELF_NAME.to_string()])
            .unwrap()
            .name = self.name.take().unwrap();
        Ok(())
    }
}

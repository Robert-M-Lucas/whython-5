use crate::processing::blocks::BlockHandler;
use crate::processing::instructions::jump_if_not_instruction_2::JumpIfNotInstruction;
use crate::processing::instructions::jump_instruction_3::JumpInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Block, Symbol};
use crate::processing::types::TypeSymbol;

pub struct IfBlock {
    jump_next_instruction: Option<JumpIfNotInstruction>,
    jump_end_instructions: Vec<JumpInstruction>,
}

impl IfBlock {
    pub fn new_block() -> Box<dyn BlockHandler> {
        Box::new(Self {
            jump_next_instruction: None,
            jump_end_instructions: Vec::new(),
        })
    }
}

impl BlockHandler for IfBlock {
    fn on_entry(
        &mut self,
        memory_managers: &mut MemoryManagers,
        reference_stack: &mut ReferenceStack,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        //? Extract condition boolean

        let condition_boolean = match handle_arithmetic_section(
            memory_managers,
            reference_stack,
            &symbol_line[1..],
            None,
            true,
        ) {
            Err(e) => return Err(e),
            Ok(value) => {
                if value.is_none() {
                    return Err("Section does not evaluate to a value".to_string());
                }
                value.unwrap()
            }
        };

        if condition_boolean.get_type() != TypeSymbol::Boolean {
            return Err(format!(
                "If expression must evaluate to {:?}",
                TypeSymbol::Boolean
            ));
        }

        //? Insert instruction to skip this section if boolean is false
        self.jump_next_instruction = Some(JumpIfNotInstruction::new_alloc(
            memory_managers,
            condition_boolean,
            0,
        ));

        Ok(())
    }

    fn on_exit(
        &mut self,
        memory_managers: &mut MemoryManagers,
        reference_stack: &mut ReferenceStack,
        symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        fn exit_with_cleanup(
            this: &mut IfBlock,
            memory_managers: &mut MemoryManagers,
            reference_stack: &mut ReferenceStack,
        ) -> Result<bool, String> {
            this.on_forced_exit(memory_managers, reference_stack)?;
            Ok(true)
        }

        //? No elif or else
        if symbol_line.is_empty() {
            return exit_with_cleanup(self, memory_managers, reference_stack);
        }

        // Filter out non-blocks
        let block_type = match &symbol_line[0] {
            Symbol::Block(block) => block,
            _ => {
                return exit_with_cleanup(self, memory_managers, reference_stack);
            }
        };

        //? Handle elif or else
        match block_type {
            Block::Elif => {
                if self.jump_next_instruction.is_none() {
                    return Err(
                        "'elif' cannot follow an 'else' block as it will never be reached"
                            .to_string(),
                    );
                }

                // Add instruction to skip to end if previous if/elif condition was met and executed
                self.jump_end_instructions
                    .push(JumpInstruction::new_alloc(memory_managers, 0));
                // Set jump next instruction to jump to this section (check this block if previous was false)
                self.jump_next_instruction
                    .as_mut()
                    .unwrap()
                    .set_destination(
                        memory_managers,
                        memory_managers.program_memory.get_position(),
                    );
                // Reuse if handling
                self.on_entry(memory_managers, reference_stack, symbol_line)?;
                // Create new scope
                reference_stack.remove_handler();
                reference_stack.add_handler();
                Ok(false)
            }
            Block::Else => {
                if symbol_line.len() > 1 {
                    return Err("Else cannot be followed by any other symbol".to_string());
                }
                if self.jump_next_instruction.is_none() {
                    return Err(
                        "'else' cannot follow an 'else' block as it will never be reached"
                            .to_string(),
                    );
                }
                // Add instruction to skip to end if previous if/elif condition was met and executed
                self.jump_end_instructions
                    .push(JumpInstruction::new_alloc(memory_managers, 0));
                // Set jump next instruction to jump to this section (run this block if previous was false)
                self.jump_next_instruction
                    .as_mut()
                    .unwrap()
                    .set_destination(
                        memory_managers,
                        memory_managers.program_memory.get_position(),
                    );
                // Else block cannot be skipped
                self.jump_next_instruction = None;
                // Create new scope
                reference_stack.remove_handler();
                reference_stack.add_handler();
                Ok(false)
            }
            _ => exit_with_cleanup(self, memory_managers, reference_stack),
        }
    }

    fn on_forced_exit(
        &mut self,
        memory_managers: &mut MemoryManagers,
        _reference_stack: &mut ReferenceStack,
    ) -> Result<(), String> {
        /*
        If :: Jump to next if not
            content
             :: Jump to end

        ElIf :: Jump to next if not
            content
             :: Jump to end

         ElIf :: Jump to next if not
            content

         */

        // Set jump to next
        if let Some(instruction) = self.jump_next_instruction.as_mut() {
            instruction.set_destination(
                memory_managers,
                memory_managers.program_memory.get_position(),
            )
        }

        // Set all jump to end
        for j in self.jump_end_instructions.iter_mut() {
            j.set_destination(
                memory_managers,
                memory_managers.program_memory.get_position(),
            );
        }
        Ok(())
    }
}

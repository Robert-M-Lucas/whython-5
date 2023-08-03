pub mod base_block;
pub mod class_block;
pub mod function_block;
pub mod if_block;
pub mod while_block;

use crate::memory::MemoryManager;
use crate::processing::blocks::base_block::BaseBlock;
use crate::processing::reference_manager::{Reference, ReferenceStack};
use crate::processing::symbols::Symbol;
use crate::util::warn;

pub enum BlockType {
    Base,
    Class,
    Function,
    If,
    While
}

pub trait BlockHandler {
    fn get_block_type(&self) -> BlockType;

    /// Enter block
    fn on_entry(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        symbol_line: &[Symbol],
    ) -> Result<(), String>;

    /// Try to exit block
    /// Returns `Ok(true)` if block exit is successful
    fn on_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
        _symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        self.on_forced_exit(program_memory, reference_stack, stack_sizes)?;
        Ok(true)
    }

    /// Force exit
    fn on_forced_exit(
        &mut self,
        program_memory: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;

    /// Break from block e.g. while
    fn on_break(&mut self, _program_memory: &mut MemoryManager) -> Result<bool, String> {
        Ok(false)
    }

    /// Continue block e.g. while
    fn on_continue(&mut self, _program_memory: &mut MemoryManager) -> Result<bool, String> {
        Ok(false)
    }

    fn update_sub_block(&mut self, block_type: Option<BlockType>) -> Result<(), String> {
        Ok(())
    }
}

pub struct StackSizes {
    sizes: Vec<usize>,
}

impl StackSizes {
    pub fn new() -> Self {
        Self { sizes: Vec::new() }
    }

    pub fn add_stack(&mut self) {
        self.sizes.push(0);
    }

    pub fn remove_stack(&mut self) {
        self.sizes.pop();
    }

    pub fn get_size(&self) -> usize {
        *self
            .sizes
            .last()
            .expect("Tried to get stack size when no stack exists")
    }

    pub fn increment_stack_size(&mut self, amount: usize) -> usize {
        let r = self.get_size();
        *self
            .sizes
            .last_mut()
            .expect("Tried to get stack size when no stack exists") += amount;
        r
    }
}

impl Default for StackSizes {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BlockCoordinator {
    stack: Vec<Box<dyn BlockHandler>>,
    stack_sizes: StackSizes,
    reference_stack: ReferenceStack,
    completed: bool,
    pub skip_sub_block_check: bool,
}

impl BlockCoordinator {
    pub fn new(program_memory: &mut MemoryManager) -> Self {
        let mut new = Self {
            stack: Vec::new(),
            stack_sizes: StackSizes::new(),
            reference_stack: ReferenceStack::new(),
            completed: false,
            skip_sub_block_check: false
        };

        // Initialise base block
        new.add_block_handler(BaseBlock::new_block(), program_memory, &[])
            .expect("Base block creation failed");

        new
    }

    pub fn complete(&mut self, program_memory: &mut MemoryManager) {
        if self.stack.len() > 1 {
            panic!("Attempted to complete BlockCoordinator when a BlockHandler is still active");
        }
        self.force_exit_block_handler(program_memory)
            .expect("Removing base block failed");
        self.completed = true;
    }

    pub fn get_stack_sizes(&mut self) -> &mut StackSizes {
        &mut self.stack_sizes
    }

    pub fn get_reference_stack_and_stack_sizes(
        &mut self,
    ) -> (&mut ReferenceStack, &mut StackSizes) {
        (&mut self.reference_stack, &mut self.stack_sizes)
    }

    /// Add a block handler
    ///
    /// # Arguments
    /// * `symbol_line` - Line that created this block
    pub fn add_block_handler(
        &mut self,
        mut handler: Box<dyn BlockHandler>,
        program_memory: &mut MemoryManager,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        self.reference_stack.add_handler();
        let (reference_stack, stack_sizes) = self.get_reference_stack_and_stack_sizes();
        let r = handler.on_entry(program_memory, reference_stack, stack_sizes, symbol_line);
        self.stack.push(handler);
        // self.stack_sizes.add_stack();
        r
    }

    /// Break from block e.g. while
    pub fn break_block_handler(
        &mut self,
        program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        let mut success = false;
        for h in self.stack.iter_mut().rev() {
            if h.on_break(program_memory)? {
                success = true;
                break;
            }
        }

        if !success {
            return Err("None of the scopes 'break' is in support breaking".to_string());
        }
        Ok(())
    }

    /// Continue block e.g. while
    pub fn continue_block_handler(
        &mut self,
        program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        let mut success = false;
        for h in self.stack.iter_mut().rev() {
            if h.on_continue(program_memory)? {
                success = true;
                break;
            }
        }

        if !success {
            return Err("None of the scopes 'continue' is in support continuing".to_string());
        }
        Ok(())
    }

    /// Try to exit block
    ///
    /// Returns `Ok(true)` if block exit is successful
    pub fn exit_block_handler(
        &mut self,
        program_memory: &mut MemoryManager,
        symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        let mut handler = self
            .stack
            .pop()
            .expect("Called on_exit when not BlockHandler exists on stack!");

        let (reference_stack, stack_sizes) = self.get_reference_stack_and_stack_sizes();

        let result = handler.on_exit(program_memory, reference_stack, stack_sizes, symbol_line);

        if let Ok(r) = result {
            return if !r {
                // Cancel stack removing
                self.stack.push(handler);
                Ok(false)
            } else {
                self.reference_stack.remove_handler();
                // self.stack_sizes.remove_stack();
                Ok(true)
            };
        }
        result
    }

    /// Force exit
    pub fn force_exit_block_handler(
        &mut self,
        program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        let mut handler = self
            .stack
            .pop()
            .expect("Called on_exit when no BlockHandler exists on stack!");

        let (reference_stack, stack_sizes) = self.get_reference_stack_and_stack_sizes();

        let result = handler.on_forced_exit(program_memory, reference_stack, stack_sizes);

        self.reference_stack.remove_handler();
        // self.stack_sizes.remove_stack();

        result
    }

    pub fn on_line_processed(&mut self) -> Result<(), String> {
        if self.skip_sub_block_check {
            self.skip_sub_block_check = false;
            return Ok(());
        }
        for i in 0..self.stack.len() {
            let block_type = self.stack.iter().nth(i + 1).map(|b| b.get_block_type());
            self.stack.iter_mut().nth(i).unwrap()
                .update_sub_block(block_type)?;
        }
        Ok(())
    }

    /// Returns the current indentation level
    pub fn get_indentation(&self) -> usize {
        // ? Subtract one for base block
        self.stack.len() - 1
    }

    /// Returns a reference to the reference stack
    pub fn get_reference_stack(&self) -> &ReferenceStack {
        &self.reference_stack
    }

    /// Returns a mutable reference to the reference stack
    pub fn get_reference_stack_mut(&mut self) -> &mut ReferenceStack {
        &mut self.reference_stack
    }

    /// Registers a variable
    pub fn register_reference(
        &mut self,
        reference: Reference,
        name: Vec<String>,
    ) -> Result<(), String> {
        self.reference_stack.register_reference(reference, name)
    }

    /// Searches for a variable going up the reference stack
    pub fn get_reference(&self, name: &[String]) -> Result<&Reference, String> {
        self.reference_stack.get_reference(name)
    }

    pub fn get_reference_and_offset(&self, name: &[String]) -> Result<(&Reference, usize), String> {
        self.reference_stack.get_reference_and_offset(name)
    }

    /// Adds a reference handler (adds a variable scope)
    pub fn add_reference_handler(&mut self) {
        self.reference_stack.add_handler()
    }

    /// Removes a reference handler (removes a variable scope)
    pub fn remove_reference_handler(&mut self) {
        self.reference_stack.remove_handler()
    }

    pub fn get_stack_sizes_and_reference_stack(
        &mut self,
    ) -> (&mut StackSizes, &mut ReferenceStack) {
        (&mut self.stack_sizes, &mut self.reference_stack)
    }
}

#[cfg(debug_assertions)]
impl Drop for BlockCoordinator {
    fn drop(&mut self) {
        if !self.completed {
            warn("BlockCoordinator dropped without 'complete' being called");
        }
    }
}

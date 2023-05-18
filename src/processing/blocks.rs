use crate::memory::MemoryManager;
use crate::processing::reference_manager::{NamedReference, ReferenceStack};
use crate::processing::symbols::Symbol;


pub trait BlockHandler {
    /// Enter block
    fn on_entry(
        &mut self,
        memory_managers: &mut MemoryManager,
        block_coordinator: &mut ReferenceStack,
        symbol_line: &[Symbol],
    ) -> Result<(), String>;

    /// Try to exit block
    ///
    /// Returns `Ok(true)` if block exit is successful
    fn on_exit(
        &mut self,
        memory_managers: &mut MemoryManager,
        reference_stack: &mut ReferenceStack,
        _symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        self.on_forced_exit(memory_managers, reference_stack)?;
        Ok(true)
    }

    /// Force exit
    fn on_forced_exit(
        &mut self,
        memory_managers: &mut MemoryManager,
        block_coordinator: &mut ReferenceStack,
    ) -> Result<(), String>;

    /// Break from block e.g. while
    fn on_break(&mut self, _memory_managers: &mut MemoryManager) -> Result<bool, String> {
        Ok(false)
    }

    /// Continue block e.g. while
    fn on_continue(&mut self, _memory_managers: &mut MemoryManager) -> Result<bool, String> {
        Ok(false)
    }
}

#[derive(Default)]
pub struct BlockCoordinator {
    stack: Vec<Box<dyn BlockHandler>>,
    reference_stack: ReferenceStack,
}

impl BlockCoordinator {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            reference_stack: ReferenceStack::new(),
        }
    }

    /// Add a block handler
    ///
    /// # Arguments
    /// * `symbol_line` - Line that created this block
    pub fn add_block_handler(
        &mut self,
        mut handler: Box<dyn BlockHandler>,
        memory_managers: &mut MemoryManager,
        symbol_line: &[Symbol],
    ) -> Result<(), String> {
        self.reference_stack.add_handler();
        let r = handler.on_entry(memory_managers, self.get_reference_stack_mut(), symbol_line);
        self.stack.push(handler);
        r
    }

    /// Break from block e.g. while
    pub fn break_block_handler(
        &mut self,
        memory_managers: &mut MemoryManager,
    ) -> Result<(), String> {
        let mut success = false;
        for h in self.stack.iter_mut().rev() {
            if h.on_break(memory_managers)? {
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
        memory_managers: &mut MemoryManager,
    ) -> Result<(), String> {
        let mut success = false;
        for h in self.stack.iter_mut().rev() {
            if h.on_continue(memory_managers)? {
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
        memory_managers: &mut MemoryManager,
        symbol_line: &[Symbol],
    ) -> Result<bool, String> {
        if self.stack.is_empty() {
            panic!("Called on_exit when not BlockHandler exists on stack!")
        }

        let mut handler = self.stack.pop().unwrap();

        let result = handler.on_exit(memory_managers, self.get_reference_stack_mut(), symbol_line);

        if let Ok(r) = result {
            return if !r {
                self.stack.push(handler);
                Ok(false)
            } else {
                self.reference_stack.remove_handler();
                Ok(true)
            };
        }
        result
    }

    /// Force exit
    pub fn force_exit_block_handler(
        &mut self,
        memory_managers: &mut MemoryManager,
    ) -> Result<(), String> {
        if self.stack.is_empty() {
            panic!("Called on_exit when not BlockHandler exists on stack!")
        }

        let mut handler = self.stack.pop().unwrap();

        let result = handler.on_forced_exit(memory_managers, self.get_reference_stack_mut());

        result
    }

    /// Returns the current indentation level
    pub fn get_indentation(&self) -> usize {
        self.stack.len()
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
    pub fn register_reference(&mut self, reference: NamedReference) -> Result<(), String> {
        self.reference_stack.register_reference(reference)
    }

    /// Searches for a variable going up the reference stack
    pub fn get_variable(&self, name: &str) -> Result<&NamedReference, String> {
        self.reference_stack.get_reference(name)
    }

    /// Adds a reference handler (adds a variable scope)
    pub fn add_reference_handler(&mut self) {
        self.reference_stack.add_handler()
    }

    /// Removes a reference handler (removes a variable scope)
    pub fn remove_reference_handler(&mut self) {
        self.reference_stack.remove_handler()
    }
}

use crate::memory::runtime_memory::dump_bytes;
use std::collections::LinkedList;
use std::fs;

pub struct StackMemory {
    memory: LinkedList<Vec<u8>>,
    current_stack: usize,
}

impl StackMemory {
    pub fn new() -> Self {
        Self {
            memory: LinkedList::new(),
            current_stack: 0,
        }
    }

    /// Creates a new stack with a specified size
    pub fn create_stack(&mut self, size: usize) {
        self.memory.push_front(vec![0; size]);
    }

    /// Returns the stack and the position in that stack of a given address
    pub fn get_stack(&self, mut position: usize) -> (&[u8], usize) {
        for m in self.memory.iter() {
            //.skip(self.memory.len() - self.current_stack) {
            if position >= m.len() {
                position -= m.len();
                continue;
            }
            return (&m, position);
        }

        panic!("Index out of stack!");
    }

    /// Returns the stack and the position in that stack of a given address
    pub fn get_stack_mut(&mut self, mut position: usize) -> (&mut [u8], usize) {
        // TODO: Optimise?
        // let len = self.memory.len();
        for m in self.memory.iter_mut() {
            //.skip(len - self.current_stack) {
            if position >= m.len() {
                position -= m.len();
                continue;
            }
            return (m, position);
        }

        panic!("Index out of stack!");
    }

    /// Returns a single byte at a given address
    pub fn index(&self, position: usize) -> u8 {
        let (stack, transformed_position) = self.get_stack(position);
        stack[transformed_position]
    }

    /// Returns a slice of the data in a stack
    pub fn index_slice(&self, mut start: usize, mut end: usize) -> &[u8] {
        for m in self.memory.iter() {
            //.skip(self.memory.len() - self.current_stack) {
            if start >= m.len() {
                start -= m.len();
                end -= m.len();
                continue;
            }
            return &m[start..end];
        }

        panic!("Index out of stack!");
    }

    /// DEPRECIATED
    pub fn stack_up(&mut self) {
        self.current_stack += 1;
    }

    /// Removes a stack
    pub fn stack_down_and_delete(&mut self) {
        let stack = self
            .memory
            .pop_front()
            .expect("Tried to stack down when there are no stacks!");
        self.current_stack -= 1;
    }

    /// Gets the current stack depth
    pub fn get_current_level(&self) -> usize {
        // self.current_stack
        self.memory.len()
    }

    /// Writes all data to a specified folder for debugging
    pub fn dump_bytes(&self, folder_name: &str) {
        fs::create_dir_all(folder_name).unwrap();
        for i in self.memory.iter().enumerate() {
            dump_bytes(
                format!("{}/stack-{}.bin", folder_name, i.0).as_str(),
                &(i.1),
            );
        }
    }
}

impl Default for StackMemory {
    fn default() -> Self {
        Self::new()
    }
}

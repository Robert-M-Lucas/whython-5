use std::collections::LinkedList;
use std::fs;
use crate::memory::runtime_memory::dump_bytes;

pub struct StackMemory {
    memory: LinkedList<(Vec<u8>, usize)>,
    current_stack: usize,
}

impl StackMemory {
    pub fn new() -> Self {
        Self {
            memory: LinkedList::new(),
            current_stack: 0,
        }
    }

    pub fn create_stack(&mut self, size: usize, return_addr: usize) {
        self.memory.push_back((vec![0; size], return_addr));
    }

    pub fn get_stack(&self, mut position: usize) -> (&[u8], usize) {
        for m in self.memory.iter().skip(self.current_stack - 1) {
            if position >= m.0.len() {
                position -= m.0.len();
                continue;
            }
            return (&m.0, position);
        }

        panic!("Index out of stack!");
    }

    pub fn get_stack_mut(&mut self, mut position: usize) -> (&mut [u8], usize) {
        for m in self.memory.iter_mut().skip(self.current_stack - 1) {
            println!("{}", m.0.len());
            println!("{}", position);
            if position >= m.0.len() {
                position -= m.0.len();
                continue;
            }
            return (&mut m.0, position);
        }

        panic!("Index out of stack!");
    }

    pub fn index(&self, mut position: usize) -> u8 {
        let (stack, transformed_position) = self.get_stack(position);
        stack[transformed_position]
    }

    pub fn index_slice(&self, mut start: usize, mut end: usize) -> &[u8] {
        for m in self.memory.iter().skip(self.current_stack - 1) {
            if start >= m.0.len() {
                start -= m.0.len();
                end -= m.0.len();
                continue;
            }
            return &m.0[start..end];
        }

        panic!("Index out of stack!");
    }

    pub fn stack_up(&mut self) {
        self.current_stack += 1;
    }

    pub fn stack_down_and_delete(&mut self) {
        self.memory.pop_back().expect("Tried to stack down when there are no stacks!");
        self.current_stack -= 1;
    }

    pub fn get_current_level(&self) -> usize { self.current_stack }

    pub fn dump_bytes(&self, folder_name: &str) {
        fs::create_dir_all(folder_name).unwrap();
        for i in self.memory.iter().enumerate() {
            dump_bytes(format!("{}/{}.bin", folder_name, i.0).as_str(), &(i.1.0));
        }
    }
}

impl Default for StackMemory {
    fn default() -> Self {
        Self::new()
    }
}
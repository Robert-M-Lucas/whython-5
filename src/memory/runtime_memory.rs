use std::collections::LinkedList;
use super::MemoryManager;

#[derive(Clone, Debug)]
pub enum MemoryLocation {
    Program,
    Stack,
    Heap,
}

pub struct StackMemory {
    memory: LinkedList<(Vec<u8>, usize)>,
    current_stack: usize,
}

impl StackMemory {
    pub fn new() -> Self {
        Self {
            memory: LinkedList::new(),
            current_stack: 0
        }
    }

    pub fn create_stack(&mut self, size: usize, return_addr: usize) {
        self.memory.push_back((vec![0; size], return_addr));
    }
    
    pub fn index(&self, mut position: usize) -> u8 {
        for m in self.memory.iter().skip(self.current_stack) {
            if position >= m.0.len() { position -= m.0.len(); continue; }
            return m.0[position];
        }

        panic!("Index out of stack!");
    }

    pub fn index_slice(&self, mut start: usize, mut end: usize) -> &[u8] {
        for m in self.memory.iter().skip(self.current_stack) {
            if start >= m.0.len() { start -= m.0.len(); end -= m.0.len(); continue; }
            return &m.0[start..end];
        }

        panic!("Index out of stack!");
    }

    pub fn get_stack(&self, mut position: usize) -> &[u8] {
        for m in self.memory.iter().skip(self.current_stack) {
            if position >= m.0.len() { position -= m.0.len(); continue; }
            return &m.0;
        }

        panic!("Index out of stack!");
    }
}

pub struct RuntimeMemoryManager {
    program_memory: Vec<u8>,
    stack_memory: StackMemory,
    heap_memory: Vec<u8>
}

impl RuntimeMemoryManager {
    pub fn from_program_memory(program_memory: MemoryManager) -> Self {
        Self {
            program_memory: program_memory.memory,
            stack_memory: StackMemory::new(),
            heap_memory: Vec::new()
        }
    }

    pub fn program_memory(&self) -> &[u8] {
        &self.program_memory
    }

    pub fn stack_memory(&mut self) -> &mut StackMemory {
        &mut self.stack_memory
    }

    pub fn heap_memory(&mut self) -> &mut Vec<u8> {
        &mut self.heap_memory
    }

    pub fn get_memory(&self, location: &MemoryLocation, start_position: usize) -> &[u8] {
        match location {
            MemoryLocation::Program => {
                &self.program_memory
            },
            MemoryLocation::Stack => {
                self.stack_memory.get_stack(start_position)
            },
            MemoryLocation::Heap => {
                &self.heap_memory
            }
        }
    }

    pub fn get_data(&self, location: &MemoryLocation, address: usize, length: usize) -> &[u8] {
        match location {
            MemoryLocation::Program => {
                &self.program_memory[address..address + length]
            },
            MemoryLocation::Stack => {
                self.stack_memory.index_slice(address, address + length)
            },
            MemoryLocation::Heap => {
                &self.heap_memory[address..address + length]
            }
        }
    }

    pub fn get_byte(&self, location: &MemoryLocation, address: usize) -> u8 {
        match location {
            MemoryLocation::Program => {
                self.program_memory[address]
            },
            MemoryLocation::Stack => {
                self.stack_memory.index(address)
            },
            MemoryLocation::Heap => {
                self.heap_memory[address]
            }
        }
    }
}
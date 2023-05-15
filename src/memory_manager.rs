use std::collections::LinkedList;
use num_format::{Locale, ToFormattedString};
use std::fs;
use std::io::Write;

use crate::util::USIZE_BYTES;

#[derive(Default)]
pub struct MemoryManager {
    pub memory: Vec<u8>,
}

impl MemoryManager {
    /// Creates an empty memory manager
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    /// Creates memory manager from vector of bytes
    pub fn from_vec(memory: Vec<u8>) -> Self {
        Self { memory }
    }

    /// Gets the position after the last piece of memory written
    pub fn get_position(&self) -> usize {
        self.memory.len()
    }

    /// Adds a byte to the memory
    pub fn append_byte(&mut self, data: u8) -> usize {
        let position = self.get_position();
        self.memory.push(data);
        position
    }

    /// Adds an array of bytes to the end
    pub fn append(&mut self, data: &[u8]) -> usize {
        let position = self.get_position();
        self.memory.extend(data);
        position
    }

    /// Overwrites a region of memory
    pub fn overwrite(&mut self, position: usize, data: &[u8]) {
        for (i, b) in data.iter().enumerate() {
            self.memory[position + i] = *b;
        }
    }

    /// Reserves a section of memory. Returns the position of this memory
    pub fn reserve(&mut self, amount: usize) -> usize {
        let position = self.get_position();
        self.memory.reserve(amount);
        for _ in 0..amount {
            self.memory.push(0);
        }
        position
    }

    /// Saves the bytes in a '`name.b`' file
    pub fn dump_bytes(&self, name: String) {
        let name = name + ".b";
        println!(
            "Dumping memory to file '{}' [{} bytes]",
            &name,
            self.memory.len().to_formatted_string(&Locale::en)
        );

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(name);

        if file.is_err() {
            println!("Failed to open file - {}", file.unwrap_err());
            return;
        }

        let mut file = file.unwrap();
        let r = file.write_all(&self.memory);
        if r.is_err() {
            println!("Failed to write to file - {}", r.unwrap_err())
        }
    }

    pub fn save_to_file(&self, name: String) {
        let name = name + format!(" - {}.cwhy", USIZE_BYTES).as_str();

        println!(
            "Saving compiled data '{}' [{} bytes]",
            &name,
            self.memory.len().to_formatted_string(&Locale::en)
        );

        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(name);

        if file.is_err() {
            println!("Failed to open file - {}", file.unwrap_err());
            return;
        }

        let mut file = file.unwrap();
        let r = file.write_all(&self.memory);
        if r.is_err() {
            println!("Failed to write to file - {}", r.unwrap_err())
        }
    }

    pub fn load_from_file(path: String) -> Result<Self, String> {
        println!("Loading precompiled data from file '{}'", &path);

        let data = match fs::read(path) {
            Err(e) => return Err(e.to_string()),
            Ok(value) => value,
        };

        Ok(
            Self{
                memory: data
            }
        )
    }
}

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

// impl Index<usize> for &StackMemory {
//     type Output = u8;
//
//     fn index(&self, mut index: usize) -> &Self::Output {
//         for m in self.memory.iter().rev() {
//             if index <= m.len() { index -= m.len(); continue; }
//             return &m[index];
//         }
//
//         panic!("Index out of stack");
//     }
// }

// impl<Idx> Index<Idx> for StackMemory
//     where
//         Idx: SliceIndex<[u8]>,
// {
//     type Output = Idx::Output;
// 
//     fn index(&self, index: Idx) -> &Self::Output {
//         for m in self.memory.iter().skip(self.current_stack) {
// 
//         }
// 
//         ()
//     }
// }

// impl<Idx> Index<Idx> for &StackMemory
//     where
//         Idx: SliceIndex<[u8]>,
// {
//     type Output = Idx::Output;
//
//     fn index(&self, index: Idx) -> &Self::Output {
//
//     }
// }

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
use std::collections::linked_list::LinkedList;
use std::fmt::format;
use std::fs;
use std::io::Write;
use super::MemoryManager;

#[derive(Clone, Debug)]
pub enum MemoryLocation {
    Program,
    Stack,
    Heap,
}

fn dump_bytes(file: &str, data: &Vec<u8>) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file).unwrap();

    file.write_all(data).unwrap();
}

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

    pub fn index(&self, mut position: usize) -> u8 {
        for m in self.memory.iter().skip(self.current_stack - 1) {
            if position >= m.0.len() {
                position -= m.0.len();
                continue;
            }
            return m.0[position];
        }

        panic!("Index out of stack!");
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

    pub fn get_stack(&self, mut position: usize) -> &[u8] {
        for m in self.memory.iter().skip(self.current_stack - 1) {
            if position >= m.0.len() {
                position -= m.0.len();
                continue;
            }
            return &m.0;
        }

        panic!("Index out of stack!");
    }

    pub fn get_stack_mut(&mut self, mut position: usize) -> &mut [u8] {
        for m in self.memory.iter_mut().skip(self.current_stack - 1) {
            println!("{}", m.0.len());
            println!("{}", position);
            if position >= m.0.len() {
                position -= m.0.len();
                continue;
            }
            return &mut m.0;
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

pub struct RuntimeMemoryManager {
    program_memory: Vec<u8>,
    stack_memory: StackMemory,
    heap_memory: Vec<u8>,
}

impl RuntimeMemoryManager {
    pub fn from_program_memory(program_memory: MemoryManager) -> Self {
        Self {
            program_memory: program_memory.memory,
            stack_memory: StackMemory::new(),
            heap_memory: Vec::new(),
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
            MemoryLocation::Program => &self.program_memory,
            MemoryLocation::Stack => self.stack_memory.get_stack(start_position),
            MemoryLocation::Heap => &self.heap_memory,
        }
    }

    pub fn get_data(&self, location: &MemoryLocation, address: usize, length: usize) -> &[u8] {
        match location {
            MemoryLocation::Program => &self.program_memory[address..address + length],
            MemoryLocation::Stack => self.stack_memory.index_slice(address, address + length),
            MemoryLocation::Heap => panic!("Heap not implemented!"),
        }
    }

    pub fn overwrite_data(&mut self, location: &MemoryLocation, address: usize, data: &[u8]) {
        match location {
            MemoryLocation::Program => {
                panic!("Overwriting program memory is forbidden!");
            },
            MemoryLocation::Stack => {
                let stack = self.stack_memory.get_stack_mut(address);
                for i in 0..data.len() {
                    stack[i] = data[i];
                }
            },
            MemoryLocation::Heap => panic!("Heap not implemented!"),
        }
    }

    pub fn get_byte(&self, location: &MemoryLocation, address: usize) -> u8 {
        match location {
            MemoryLocation::Program => self.program_memory[address],
            MemoryLocation::Stack => self.stack_memory.index(address),
            MemoryLocation::Heap => self.heap_memory[address],
        }
    }

    pub fn dump_all(&self) {
        let dir_name = "dump";
        fs::create_dir_all(dir_name).unwrap();
        dump_bytes(format!("{}/program.bin", dir_name).as_str(), &self.program_memory);
        self.stack_memory.dump_bytes(format!("{}/stack", dir_name).as_str());
    }
}

mod stack_memory;
mod heap_memory;

pub use stack_memory::StackMemory;
pub use stack_memory::HeapMemory;

use std::fs;
use std::io::Write;
use super::MemoryManager;

#[derive(Clone, Debug)]
pub enum MemoryLocation {
    Program,
    Stack,
    Heap(usize), // ? frame: usize
}

fn dump_bytes(file: &str, data: &Vec<u8>) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file).unwrap();

    file.write_all(data).unwrap();
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

    /// Returns a reference to the memory as `&[u8]` and the transformed address location as a
    /// `usize`. See `StackMemory::get_stack` for details about how the address location is
    /// transformed
    pub fn get_memory(&self, location: &MemoryLocation, start_position: usize) -> (&[u8], usize) {
        match location {
            MemoryLocation::Program => (&self.program_memory, start_position),
            MemoryLocation::Stack => self.stack_memory.get_stack(start_position),
            MemoryLocation::Heap => (&self.heap_memory, start_position),
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
                let (stack, stack_address) = self.stack_memory.get_stack_mut(address);
                for i in 0..data.len() {
                    stack[stack_address + i] = data[i];
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

use num_format::{Locale, ToFormattedString};
use std::fs;
use std::io::Write;

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
}

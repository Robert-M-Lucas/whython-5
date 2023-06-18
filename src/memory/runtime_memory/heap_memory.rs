use std::collections::LinkedList;
use std::fs;
use crate::memory::runtime_memory::dump_bytes;

pub struct HeapMemory {
    // TODO: Use more efficient data structure (hash table?)
    // (frame id, data)
    memory: LinkedList<(usize, Vec<u8>)>,
    next_frame: usize
}


impl HeapMemory {
    pub fn new() -> Self {
        Self {
            memory: LinkedList::new(),
            next_frame: 0
        }
    }

    // TODO: Better frame creation algorithm (prevent allow id re-use, prevent double usage)
    /// Creates frame with specified size, returns frame id
    pub fn create_frame(&mut self, size: usize) -> usize {
        self.memory.push_back((self.next_frame, vec![0; size]));
        self.next_frame += 1;
        self.next_frame - 1
    }

    pub fn get_frame(&self, frame: usize) -> &[u8] {
        for f in self.memory.iter() {
            if f.0 == frame {
                return &f.1;
            }
        }

        panic!("Frame not in Heap!");
    }

    pub fn get_mut_frame(&mut self, frame: usize) -> &mut [u8] {
        for f in self.memory.iter_mut() {
            if f.0 == frame {
                return &mut f.1;
            }
        }

        panic!("Frame not in Heap!");
    }

    pub fn index(&self, frame: usize, position: usize) -> u8 {
        self.get_frame(frame)[position]
    }

    pub fn index_slice(&self, frame: usize, start: usize, end: usize) -> &[u8] {
        &self.get_frame(frame)[start..end]
    }

    pub fn dump_bytes(&self, folder_name: &str) {
        fs::create_dir_all(folder_name).unwrap();
        for i in self.memory.iter() {
            dump_bytes(format!("{}/{}.bin", folder_name, i.0).as_str(), &(i.1));
        }
    }
}

impl Default for HeapMemory {
    fn default() -> Self {
        Self::new()
    }
}

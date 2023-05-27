use std::fmt::Formatter;
use std::fmt::Debug;
use crate::memory::{MemoryLocation, RuntimeMemoryManager};
use crate::util::{get_usize, USIZE_BYTES};

pub struct CloneableBox<T> where T: Clone + Debug {
    inner_box: Box<T>
}

impl<T: Clone + Debug> CloneableBox<T> {
    pub fn new(inner: T) -> CloneableBox<T> {
        Self { inner_box: Box::new(inner) }
    }
}

impl<T: Clone + Debug> CloneableBox<T> {
    pub fn get_box_ref(&self) -> &Box<T> {
        &self.inner_box
    }

    pub fn get_box(self) -> Box<T> {
        self.inner_box
    }
}

impl<T> Clone for CloneableBox<T> where T: Clone + Debug {
    fn clone(&self) -> Self {
        Self { inner_box: Box::new(*self.inner_box.clone()) }
    }
}

impl<T> Debug for CloneableBox<T> where T: Clone + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner_box.fmt(f)
    }
}



#[derive(Clone, Debug)]
pub enum Address {
    Immediate(Vec<u8>),
    StackDirect(usize),
    StackIndirect(usize),
    HeapDirect((usize, usize)),
    HeapIndirect((usize, usize)),
    Indexed(CloneableBox<Address>, CloneableBox<Address>), // Location, Offset
}

const IMMEDIATE_CODE: u8 = 0;
const STACK_DIRECT_CODE: u8 = 1;
const STACK_INDIRECT_CODE: u8 = 2;
const HEAP_DIRECT_CODE: u8 = 3;
const HEAP_INDIRECT_CODE: u8 = 4;
const INDEXED_CODE: u8 = 5;

impl Address {
    pub fn is_immediate(&self) -> bool {
        match self {
            Address::Immediate(_) => true,
            _ => false
        }
    }

    pub fn get_address_size(memory: &[u8], address: usize, expected_len: usize) -> usize {
        match memory[address] {
            IMMEDIATE_CODE => 1 + expected_len,
            STACK_DIRECT_CODE | STACK_INDIRECT_CODE => 1 + USIZE_BYTES,
            HEAP_DIRECT_CODE | HEAP_INDIRECT_CODE => 1 + (USIZE_BYTES * 2),
            INDEXED_CODE => {
                let mut p = address + 1;
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p - address
            },
            _ => panic!("Invalid address code!")
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            Address::Immediate(data) => {
                let mut v = vec![IMMEDIATE_CODE];
                v.extend(data.iter());
                v
            },
            Address::StackDirect(address) => {
                let mut v = vec![STACK_DIRECT_CODE];
                v.append(&mut Vec::from(address.to_le_bytes()));
                v
            },
            Address::StackIndirect(address) => {
                let mut v = vec![STACK_INDIRECT_CODE];
                v.append(&mut Vec::from(address.to_le_bytes()));
                v
            },
            Address::HeapDirect(address) => {
                let mut v = vec![HEAP_DIRECT_CODE];
                v.append(&mut Vec::from(address.0.to_le_bytes()));
                v.append(&mut Vec::from(address.1.to_le_bytes()));
                v
            },
            Address::HeapIndirect(address) => {
                let mut v = vec![HEAP_INDIRECT_CODE];
                v.append(&mut Vec::from(address.0.to_le_bytes()));
                v.append(&mut Vec::from(address.1.to_le_bytes()));
                v
            },
            Address::Indexed(location, offset) => {
                let mut v = vec![INDEXED_CODE];
                v.append(&mut location.get_box_ref().get_bytes());
                v.append(&mut offset.get_box_ref().get_bytes());
                v
            },
        }
    }

    pub fn evaluate_address(pointer: &mut usize, expected_len: &usize, memory: &RuntimeMemoryManager, address_location: &MemoryLocation) -> (usize, MemoryLocation) {
        let code = memory.get_byte(address_location, *pointer);
        *pointer += 1;

        match code {
            IMMEDIATE_CODE => {
                *pointer += expected_len;
                (*pointer - expected_len, address_location.clone())
            },
            STACK_DIRECT_CODE => {
                (get_usize(pointer, memory.get_memory(address_location, *pointer)), MemoryLocation::Stack)
            },
            STACK_INDIRECT_CODE => {
                let next_address = get_usize(pointer, memory.get_memory(address_location, *pointer));
                let saved_pointer = *pointer; // Save pointer location to return to once address is found

                *pointer = next_address;
                let address = Self::evaluate_address(pointer, expected_len, memory, &MemoryLocation::Stack);
                *pointer = saved_pointer;

                address
            },
            HEAP_DIRECT_CODE => {
                (get_usize(pointer, memory.program_memory()), MemoryLocation::Heap)
            },
            HEAP_INDIRECT_CODE => {
                let next_address = get_usize(pointer, memory.get_memory(address_location, *pointer));
                let saved_pointer = *pointer; // Save pointer location to return to once address is found

                *pointer = next_address;
                let address = Self::evaluate_address(pointer, expected_len, memory, &MemoryLocation::Heap);
                *pointer = saved_pointer;

                address
            },
            INDEXED_CODE => {
                let location_address = Self::evaluate_address(pointer, &USIZE_BYTES, memory, address_location);
                let location = get_usize(&mut location_address.0.clone(), memory.get_memory(&location_address.1, location_address.0));

                let offset_address = Self::evaluate_address(pointer, &USIZE_BYTES, memory, address_location);
                let offset = get_usize(&mut offset_address.0.clone(), memory.get_memory(&offset_address.1, offset_address.0));

                (location + (offset * expected_len), location_address.1)
            },
            _ => panic!("Invalid address code!")
        }
    }
}
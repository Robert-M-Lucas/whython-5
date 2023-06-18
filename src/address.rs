use crate::memory::{MemoryLocation, RuntimeMemoryManager};
use crate::util::{get_usize, USIZE_BYTES};
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct CloneableBox<T>
where
    T: Clone + Debug,
{
    inner_box: Box<T>,
}

impl<T: Clone + Debug> CloneableBox<T> {
    pub fn new(inner: T) -> CloneableBox<T> {
        Self {
            inner_box: Box::new(inner),
        }
    }
}

impl<T: Clone + Debug> CloneableBox<T> {
    pub fn get_ref(&self) -> &T {
        self.inner_box.as_ref()
    }

    pub fn get(self) -> Box<T> {
        self.inner_box
    }
}

impl<T> Clone for CloneableBox<T>
where
    T: Clone + Debug,
{
    fn clone(&self) -> Self {
        Self {
            inner_box: Box::new(*self.inner_box.clone()),
        }
    }
}

impl<T> Debug for CloneableBox<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner_box.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub enum Address {
    Immediate(Vec<u8>),
    ImmediateIndexed(CloneableBox<Address>, CloneableBox<Address>),
    StackDirect(usize),
    StackIndirect(usize),
    StackIndexed(CloneableBox<Address>, CloneableBox<Address>),
    HeapDirect((usize, usize)),
    HeapIndirect((usize, usize)),
    HeapIndexed(CloneableBox<Address>, CloneableBox<Address>, CloneableBox<Address>),
}

const IMMEDIATE_CODE: u8 = 0;
const IMMEDIATE_INDEXED_CODE: u8 = 1;
const STACK_DIRECT_CODE: u8 = 2;
const STACK_INDIRECT_CODE: u8 = 3;
const STACK_INDEXED_CODE: u8 = 4;
const HEAP_DIRECT_CODE: u8 = 5;
const HEAP_INDIRECT_CODE: u8 = 6;
const HEAP_INDEXED_CODE: u8 = 7;


impl Address {
    pub fn is_immediate(&self) -> bool {
        matches!(self, Address::Immediate(_))
    }

    pub fn get_address_size(memory: &[u8], address: usize, expected_len: usize) -> usize {
        match memory[address] {
            // ? Code + length
            IMMEDIATE_CODE => 1 + expected_len,
            // ? Code + address length
            STACK_DIRECT_CODE | STACK_INDIRECT_CODE => 1 + USIZE_BYTES,
            // ? Code + heap frame length + address length
            HEAP_DIRECT_CODE | HEAP_INDIRECT_CODE => 1 + USIZE_BYTES + USIZE_BYTES,
            //? Code + location address length + offset address length
            IMMEDIATE_INDEXED_CODE | STACK_INDEXED_CODE => {
                let mut p = address + 1;
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p - address
            }
            //? Code + frame address length + location address length + offset address length
            HEAP_INDEXED_CODE => {
                let mut p = address + 1;
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p += Self::get_address_size(memory, p, USIZE_BYTES);
                p - address
            }
            _ => panic!("Invalid address code!"),
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            Address::Immediate(data) => {
                let mut v = vec![IMMEDIATE_CODE];
                v.extend(data.iter());
                v
            },
            Address::ImmediateIndexed(location, offset) => {
                let mut v = vec![IMMEDIATE_INDEXED_CODE];
                v.append(&mut location.get_ref().get_bytes());
                v.append(&mut offset.get_ref().get_bytes());
                v
            }
            Address::StackDirect(address) => {
                let mut v = vec![STACK_DIRECT_CODE];
                v.append(&mut Vec::from(address.to_le_bytes()));
                v
            }
            Address::StackIndirect(address) => {
                let mut v = vec![STACK_INDIRECT_CODE];
                v.append(&mut Vec::from(address.to_le_bytes()));
                v
            },
            Address::StackIndexed(location, offset) => {
                let mut v = vec![STACK_INDEXED_CODE];
                v.append(&mut location.get_ref().get_bytes());
                v.append(&mut offset.get_ref().get_bytes());
                v
            }
            Address::HeapDirect(address) => {
                let mut v = vec![HEAP_DIRECT_CODE];
                v.append(&mut Vec::from(address.0.to_le_bytes()));
                v.append(&mut Vec::from(address.1.to_le_bytes()));
                v
            }
            Address::HeapIndirect(address) => {
                let mut v = vec![HEAP_INDIRECT_CODE];
                v.append(&mut Vec::from(address.0.to_le_bytes()));
                v.append(&mut Vec::from(address.1.to_le_bytes()));
                v
            }
            Address::HeapIndexed(frame, location, offset) => {
                let mut v = vec![HEAP_INDEXED_CODE];
                v.append(&mut frame.get_ref().get_bytes());
                v.append(&mut location.get_ref().get_bytes());
                v.append(&mut offset.get_ref().get_bytes());
                v
            }
        }
    }

    // TODO: Properly support heap memory
    /// Evaluates a pointer to find the final data it points to.
    ///
    /// # Arguments
    /// - `pointer`: Represents address location - moved to point to end of address
    /// - `expected_len`: Length of the data. Ignored in direct and indirect addressing.
    /// In indexed addressing refers to the size of a single item
    /// - `address_location`: Location of the address to evaluate
    pub fn evaluate_address(
        pointer: &mut usize,
        address_location: &MemoryLocation,
        expected_len: &usize,
        memory: &RuntimeMemoryManager,
    ) -> (usize, MemoryLocation) {
        let code = memory.get_byte(address_location, *pointer);
        *pointer += 1;

        match code {
            IMMEDIATE_CODE => {
                // Increment pointer by immediate length, return start of immediate
                // and the same location
                *pointer += expected_len;
                (*pointer - expected_len, address_location.clone())
            }
            STACK_DIRECT_CODE => {
                // Get pointer
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(address_location, *pointer);

                // ? Increment real pointer
                *pointer += USIZE_BYTES;

                // ? Return location (doesn't increment real pointer)
                (
                    get_usize(&mut transformed_pointer, address_memory),
                    MemoryLocation::Stack,
                )
            },
            STACK_INDIRECT_CODE => {
                // Get pointer
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(address_location, *pointer);

                // ? Get next address (doesn't increment real pointer)
                let mut next_address = get_usize(&mut transformed_pointer, address_memory);

                // ? Increment real pointer by usize
                *pointer += USIZE_BYTES;

                // Recursively get address
                let address =
                    Self::evaluate_address(
                        &mut next_address,
                        &MemoryLocation::Stack,
                        expected_len,
                        memory
                    );

                address
            }
            HEAP_DIRECT_CODE => {
                // Get frame pointer
                let (frame_memory, mut frame_pointer) =
                    memory.get_memory(address_location, *pointer);

                // Get address pointer
                let (address_memory, mut address_pointer) =
                    memory.get_memory(address_location, *pointer + USIZE_BYTES);

                // ? Increment real pointer
                *pointer += USIZE_BYTES + USIZE_BYTES;

                // ? Return location (doesn't increment real pointer)
                (
                    get_usize(&mut address_pointer, address_memory),
                    MemoryLocation::Heap(get_usize(&mut frame_pointer, frame_memory)),
                )
            },
            HEAP_INDIRECT_CODE => {
                // Get frame pointer
                let (frame_memory, mut frame_pointer) =
                    memory.get_memory(address_location, *pointer);

                // Get address pointer
                let (address_memory, mut address_pointer) =
                    memory.get_memory(address_location, *pointer + USIZE_BYTES);

                // ? Get next frame (doesn't increment real pointer)
                let next_frame = get_usize(&mut frame_pointer, frame_memory);

                // ? Get next address (doesn't increment real pointer)
                let mut next_address = get_usize(&mut address_pointer, address_memory);

                // ? Increment real pointer by usize
                *pointer += USIZE_BYTES + USIZE_BYTES;

                // Recursively get address
                let address =
                    Self::evaluate_address(
                        &mut next_address,
                        &MemoryLocation::Heap(next_frame),
                        expected_len,
                        memory
                    );

                address
            }
            IMMEDIATE_INDEXED_CODE | STACK_INDEXED_CODE => {
                // Get location address using normal evaluate
                let (location_address, location_memory_location) =
                    Self::evaluate_address(
                        pointer,
                        address_location,
                        &USIZE_BYTES, // ? Expecting usize (address)
                        memory
                    ); // ? pointer incremented here

                // Get memory at location address
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(&location_memory_location, location_address);

                // Get location
                let location = get_usize(&mut transformed_pointer, address_memory);


                // Get offset address using normal evaluate
                let (offset_address, offset_memory_location) =
                    Self::evaluate_address(
                        pointer,
                        address_location,
                        &USIZE_BYTES, // ? Expecting usize (address)
                        memory
                    ); // ? pointer incremented here

                // Get memory at offset address
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(&offset_memory_location, offset_address);

                // Get offset
                let offset = get_usize(&mut transformed_pointer, address_memory);


                (
                    location + (offset * expected_len),
                    match code {
                        IMMEDIATE_INDEXED_CODE => MemoryLocation::Program,
                        STACK_INDEXED_CODE => MemoryLocation::Stack,
                        _ => panic!()
                    }
                )
            },
            HEAP_INDEXED_CODE => {
                // Get frame address using normal evaluate
                let (frame_address, frame_memory_location) =
                    Self::evaluate_address(
                        pointer,
                        address_location,
                        &USIZE_BYTES, // ? Expecting usize (address)
                        memory
                    ); // ? pointer incremented here

                // Get memory at frame address
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(&frame_memory_location, frame_address);

                // Get frame
                let frame = get_usize(&mut transformed_pointer, address_memory);

                // Get location address using normal evaluate
                let (location_address, location_memory_location) =
                    Self::evaluate_address(
                        pointer,
                        address_location,
                        &USIZE_BYTES, // ? Expecting usize (address)
                        memory
                    ); // ? pointer incremented here

                // Get memory at location address
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(&location_memory_location, location_address);

                // Get location
                let location = get_usize(&mut transformed_pointer, address_memory);


                // Get offset address using normal evaluate
                let (offset_address, offset_memory_location) =
                    Self::evaluate_address(
                        pointer,
                        address_location,
                        &USIZE_BYTES, // ? Expecting usize (address)
                        memory
                    ); // ? pointer incremented here

                // Get memory at offset address
                let (address_memory, mut transformed_pointer) =
                    memory.get_memory(&offset_memory_location, offset_address);

                // Get offset
                let offset = get_usize(&mut transformed_pointer, address_memory);


                (
                    location + (offset * expected_len),
                    MemoryLocation::Heap(frame)
                )
            }
            _ => panic!("Invalid address code!"),
        }
    }
}

use crate::address::Address;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::reference_manager::function::FunctionReference;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::Type;

pub struct ClassReference {
    pub name: String,
    properties: Vec<(String, TypeSymbol)>,
    functions: Vec<(String, FunctionReference)>,
}


impl ClassReference {
    pub fn new_empty(name: String) -> Self {
        Self {
            name,
            properties: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn as_type(&self) -> &dyn Type {
        self as &dyn Type
    }
}

impl Type for ClassReference {
    fn get_type_symbol(&self) -> TypeSymbol {
        todo!()
    }

    fn runtime_copy_from(&self, other: &dyn Type, program_memory: &mut MemoryManager) -> Result<CopyInstruction, String> {
        todo!()
    }

    fn runtime_copy_from_literal(&self, literal: &Literal, program_memory: &mut MemoryManager) -> Result<CopyInstruction, String> {
        todo!()
    }

    fn get_prefix_operation_result_type(&self, operator: &Operator) -> Vec<TypeSymbol> {
        todo!()
    }

    fn get_operation_result_type(&self, operator: &Operator, rhs: &TypeSymbol) -> Vec<TypeSymbol> {
        todo!()
    }

    fn operate_prefix(&self, operator: &Operator, destination: &dyn Type, program_memory: &mut MemoryManager, stack_sizes: &mut StackSizes) -> Result<(), String> {
        todo!()
    }

    fn operate(&self, operator: &Operator, rhs: &dyn Type, destination: &dyn Type, program_memory: &mut MemoryManager, stack_sizes: &mut StackSizes) -> Result<(), String> {
        todo!()
    }

    fn get_address(&self) -> &Address {
        todo!()
    }

    fn get_length(&self) -> usize {
        todo!()
    }

    fn get_address_mut(&mut self) -> &mut Address {
        todo!()
    }

    fn duplicate(&self) -> Box<dyn Type> {
        todo!()
    }
}

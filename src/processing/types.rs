use crate::processing::symbols::{Literal, Operator, TypeSymbol};

use self::boolean::BoolWrapper;

mod defaults;
use crate::address::Address;
use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
pub use defaults::*;

pub mod boolean;

pub trait UninstantiatedType {
    fn instantiate(&self) -> Box<dyn Type>;

    fn get_type_symbol(&self) -> TypeSymbol;
}

pub trait Type {
    fn get_type_symbol(&self) -> TypeSymbol;

    fn allocate_variable(
        &mut self,
        _stack: &mut StackSizes,
        _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        Err(format!(
            "{:?} cannot be allocated as a variable",
            self.get_type_symbol()
        ))
    }

    fn get_constant(&self, _literal: &Literal) -> Result<Address, String> {
        Err(format!(
            "{:?} cannot be created as a constant",
            self.get_type_symbol()
        ))
    }

    fn runtime_copy_from(&self, other: &Box<dyn Type>) -> Result<(), String>;

    fn runtime_copy_from_literal(
        &self,
        literal: &Literal,
        program_memory: &mut MemoryManager,
    ) -> Result<(), String>;

    fn get_prefix_operation_result_type(&self, operator: &Operator) -> Vec<TypeSymbol>;

    fn get_operation_result_type(&self, operator: &Operator, rhs: &TypeSymbol) -> Vec<TypeSymbol>;

    fn operate_prefix(
        &self,
        operator: &Operator,
        destination: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;

    fn operate(
        &self,
        operator: &Operator,
        rhs: &Box<dyn Type>,
        destination: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;

    fn get_address_and_length(&self) -> (&Address, usize);

    fn run_method(
        &self,
        method_name: &String,
        _stack: &mut StackSizes,
        _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        Err(format!(
            "'{}' not implemented for {:?}",
            method_name,
            self.get_type_symbol()
        ))
    }
}

pub trait Operation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self, rhs: &TypeSymbol) -> Option<TypeSymbol>;

    fn operate(
        &self,
        lhs: &LHS,
        rhs: &Box<dyn Type>,
        destination: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;
}

pub trait PrefixOperation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self) -> Option<TypeSymbol>;

    fn operate_prefix(
        &self,
        lhs: &LHS,
        destination: &Box<dyn Type>,
        program_memory: &mut MemoryManager,
        stack_sizes: &mut StackSizes,
    ) -> Result<(), String>;
}

// TODO: Refine
pub struct TypeFactory {
    uninstantiated_types: Vec<Box<dyn UninstantiatedType>>,
}

impl TypeFactory {
    pub fn get() -> Self {
        Self {
            uninstantiated_types: vec![Box::new(BoolWrapper {})],
        }
    }

    pub fn get_unallocated_type(new_type: &TypeSymbol) -> Result<Box<dyn Type>, String> {
        let factory = Self::get();
        let Some(wrapper) = factory.uninstantiated_types.iter()
            .find(|t| t.get_type_symbol() == *new_type)
        else { return Err(format!("Type {:?} cannot be instantiated", new_type)); };

        return Ok(wrapper.instantiate());
    }

    pub fn get_default_type_for_literal(literal: &Literal) -> Result<TypeSymbol, String> {
        match literal {
            Literal::Bool(_) => Ok(TypeSymbol::Boolean),
            _ => Err(format!(
                "{} does not have a default type (use as syntax)",
                literal
            )),
        }
    }

    pub fn get_default_instantiated_type_for_literal(
        literal: &Literal,
        stack: &mut StackSizes,
        program_memory: &mut MemoryManager,
    ) -> Result<Box<dyn Type>, String> {
        let type_symbol = Self::get_default_type_for_literal(literal)?;
        let mut t = Self::get_unallocated_type(&type_symbol)?;
        t.allocate_variable(stack, program_memory)?;
        t.runtime_copy_from_literal(literal, program_memory)?;
        Ok(t)
    }
}

impl Default for TypeFactory {
    fn default() -> Self {
        Self::get()
    }
}

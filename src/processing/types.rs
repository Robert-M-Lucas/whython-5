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
        _to_assign: Option<&Literal>,
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

    fn operate(&self, rhs: Box<dyn Type>) -> Result<(), String>;

    fn run_method(&self,
                  method_name: &String,
                  _stack: &mut StackSizes,
                  _program_memory: &mut MemoryManager,
    ) -> Result<(), String> {
        Err(format!("'{}' not implemented for {}", method_name, self.get_type_symbol()))
    }
}

pub trait Operation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self, rhs: Option<TypeSymbol>) -> Option<TypeSymbol>;

    fn operate(&self, lhs: &LHS, rhs: Box<dyn Type>) -> Result<(), String>;
}

pub struct TypeFactory {
    uninstantiated_types: Vec<Box<dyn UninstantiatedType>>,
}

impl TypeFactory {
    pub fn new() -> Self {
        Self {
            uninstantiated_types: vec![Box::new(BoolWrapper {})],
        }
    }

    pub fn get_unallocated_type(&self, new_type: &TypeSymbol) -> Result<Box<dyn Type>, String> {
        let Some(wrapper) = self.uninstantiated_types.iter()
            .find(|t| t.get_type_symbol() == *new_type)
        else { return Err(format!("Type {:?} cannot be instantiated", new_type)); };

        return Ok(wrapper.instantiate());
    }

    pub fn get_default_type_for_literal(literal: &Literal) -> Result<TypeSymbol, String> {
        match literal {
            Literal::Bool(_) => Ok(TypeSymbol::Boolean),
            _ => Err(format!("{} does not have a default type (use as syntax)", literal))
        }
    }
}

impl Default for TypeFactory {
    fn default() -> Self {
        Self::new()
    }
}

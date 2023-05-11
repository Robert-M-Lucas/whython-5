pub mod boolean;
pub mod char;
pub mod function;
pub mod pointer;

use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::dynamic_from_copy_instruction_10::DynamicFromCopyInstruction;
use crate::processing::instructions::dynamic_to_copy_instruction_11::DynamicToCopyInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, Symbol, SymbolHandler};
use crate::processing::types::boolean::BooleanType;
use crate::processing::types::char::CharType;
use crate::processing::types::pointer::PointerType;

macro_rules! create_type {
    ($internal_type: ident, $memory_managers: expr) => {
        Type::new(Box::new($internal_type::create_empty()), $memory_managers)
    };
}

/// Converts a `TypeSymbol` to an instantiated `Type`
pub fn get_type(
    type_symbol: &TypeSymbol,
    memory_managers: &mut MemoryManagers,
) -> Result<Type, String> {
    match type_symbol {
        TypeSymbol::Boolean => Ok(create_type!(BooleanType, memory_managers)),
        TypeSymbol::Character => Ok(create_type!(CharType, memory_managers)),
        TypeSymbol::Pointer => Ok(create_type!(PointerType, memory_managers)),
        type_symbol => Err(format!(
            "{:?}(s) cannot be created! (Are you trying to operate on an invalid type?)",
            type_symbol
        )),
    }
}

/// Converts a `Literal` to the default `Type` for that type of `Literal`
pub fn get_type_from_literal(
    literal: &Literal,
    memory_managers: &mut MemoryManagers,
) -> Result<Type, String> {
    match literal {
        Literal::Bool(_) => Ok(create_type!(BooleanType, memory_managers)),
        Literal::String(_) => Ok(create_type!(CharType, memory_managers)),
        Literal::Int(_) => Ok(create_type!(PointerType, memory_managers)),
        _ => Err(format!("Cannot infer type from {}", literal)),
    }
}

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum TypeSymbol {
    Integer,
    Boolean,
    Character,
    Function,
    Pointer,
}

pub struct TypeSymbolHandler {}

impl SymbolHandler for TypeSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        match string {
            "int" => Some(Symbol::Type(TypeSymbol::Integer)),
            "bool" => Some(Symbol::Type(TypeSymbol::Boolean)),
            "char" => Some(Symbol::Type(TypeSymbol::Character)),
            "ptr" => Some(Symbol::Type(TypeSymbol::Pointer)),
            _ => None,
        }
    }
}

pub struct Type {
    internal_type: Box<dyn TypeTrait>,
    name: Option<String>,
    address: usize,
    indexed_len: Option<usize>,
}

impl Type {
    pub fn new(internal_type: Box<dyn TypeTrait>, memory_managers: &mut MemoryManagers) -> Self {
        let address = memory_managers
            .variable_memory
            .reserve(internal_type.get_size());

        Self {
            internal_type,
            name: None,
            address,
            indexed_len: None,
        }
    }

    /// Sets the name of the `Type`
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    /// Gets the name of the `Type`
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }

    /// Gets whether the `Type` is an indexed one
    pub fn is_indexed(&self) -> bool {
        self.indexed_len.is_some()
    }

    /// Returns the length of an indexed `Type`. Panics if type is not indexed.
    pub fn get_len(&self) -> usize {
        self.indexed_len.unwrap_or(1)
    }

    /// Assigns to `Type` from another `Type`
    pub fn assign_clone(
        &self,
        memory_managers: &mut MemoryManagers,
        to_clone: &Type,
    ) -> Result<(), String> {
        if self.is_indexed() {
            return Err("Tried to assign to type that needs indexing".to_string());
        }
        self.internal_type
            .assign_clone(self, memory_managers, to_clone)
    }

    /// Assigns to `Type` from a `Literal`
    pub fn static_assign_literal(
        &self,
        memory_managers: &mut MemoryManagers,
        literal: &Literal,
    ) -> Result<(), String> {
        if self.is_indexed() {
            return Err("Tried to assign to type that needs indexing".to_string());
        }
        self.internal_type
            .static_assign_literal(self, memory_managers, literal)
    }

    /// Creates an indexed `Type`
    pub fn create_indexed(
        &mut self,
        _memory_managers: &mut MemoryManagers,
        _argument_literal: &Literal,
        _assignment_literal: &Literal,
    ) -> Result<usize, String> {
        let result = self.internal_type.create_indexed(
            self,
            _memory_managers,
            _argument_literal,
            _assignment_literal,
        );
        result.as_ref()?;
        self.indexed_len = Some(*result.as_ref().unwrap());
        result
    }

    /// Gets the value at an index and assigns it to the `destination`
    pub fn get_indexed(
        &self,
        memory_managers: &mut MemoryManagers,
        index_pointer: &Type,
        destination: &Type,
    ) -> Result<(), String> {
        if !self.is_indexed() {
            return Err("Tried to index type that isn't indexed".to_string());
        }

        self.internal_type
            .get_index(self, memory_managers, index_pointer, destination)
    }

    /// Sets the value at an index to `source`
    pub fn set_indexed(
        &self,
        memory_managers: &mut MemoryManagers,
        index_pointer: &Type,
        source: &Type,
    ) -> Result<(), String> {
        if !self.is_indexed() {
            return Err("Tried to index type that isn't indexed".to_string());
        }

        self.internal_type
            .set_index(self, memory_managers, index_pointer, source)
    }

    /// Gets the `TypeSymbol` corresponding to this `Type`
    pub fn get_type(&self) -> TypeSymbol {
        self.internal_type.get_type()
    }

    /// Gets return type if this `Type` can be called
    pub fn get_return_type(&self) -> Result<TypeSymbol, String> {
        self.internal_type.get_return_type()
    }

    /// Gets the variable memory address of this `Type`
    pub fn get_address(&self) -> usize {
        self.address
    }

    /// Gets the size of this `Type`
    pub fn get_size(&self) -> usize {
        self.internal_type.get_size()
    }

    /// Calls this `Type` if it can be called. Put the return value into `destination` if it is not `None`
    pub fn call(
        &self,
        memory_managers: &mut MemoryManagers,
        arguments: Vec<&Type>,
        destination: Option<&Type>,
    ) -> Result<(), String> {
        self.internal_type
            .call(memory_managers, arguments, destination)
    }

    /// Gets the `TypeSymbol` that the given operation would return
    pub fn get_operation_return_type(
        &self,
        operator: &Operator,
        rhs: Option<&Type>,
    ) -> Result<TypeSymbol, String> {
        self.internal_type.get_operation_type(self, operator, rhs)
    }

    /// Performs `operator` on `self` and `rhs` (if it is `Some`). Puts result in `destination`
    pub fn operate(
        &self,
        memory_managers: &mut MemoryManagers,
        operator: Operator,
        rhs: Option<&Type>,
        destination: &Type,
    ) -> Result<(), String> {
        self.internal_type
            .operate(self, memory_managers, operator, rhs, destination)
    }

    /// Clones the `Type`
    pub fn duplicate(&self) -> Self {
        Self {
            internal_type: self.internal_type.clone(),
            name: self.name.clone(),
            address: self.address,
            indexed_len: self.indexed_len,
        }
    }
}

pub trait TypeTrait {
    /// Assigns to `Type` from another `Type`
    fn assign_clone(
        &self,
        _super: &Type,
        memory_managers: &mut MemoryManagers,
        to_clone: &Type,
    ) -> Result<(), String> {
        if self.get_type() != to_clone.get_type() {
            return Err(format!(
                "Mismatching types for assignment: {} -> {}",
                to_clone.get_type(),
                self.get_type()
            ));
        }

        CopyInstruction::new_alloc(
            memory_managers,
            to_clone.get_address(),
            _super.get_address(),
            self.get_size(),
        );

        Ok(())
    }

    /// Assigns to `Type` from a `Literal`
    fn static_assign_literal(
        &self,
        _super: &Type,
        _memory_managers: &mut MemoryManagers,
        _literal: &Literal,
    ) -> Result<(), String> {
        Err(format!(
            "Assignment from literals not implemented for {}",
            self.get_type()
        ))
    }

    /// Creates an indexed `Type`
    fn create_indexed(
        &self,
        _super: &Type,
        _memory_managers: &mut MemoryManagers,
        _argument_literal: &Literal,
        _assignment_literal: &Literal,
    ) -> Result<usize, String> {
        Err(format!(
            "{} cannot be created with initialisation argument",
            self.get_type()
        ))
    }

    /// Gets the value at an index and assigns it to the `destination`
    fn get_index(
        &self,
        _super: &Type,
        memory_managers: &mut MemoryManagers,
        index_pointer: &Type,
        destination: &Type,
    ) -> Result<(), String> {
        match index_pointer.get_type() {
            TypeSymbol::Pointer => {}
            _ => return Err("Only pointers are supported for indexing this type".to_string()),
        }

        if self.get_type() != destination.get_type() {
            return Err(format!(
                "Cannot move value from indexed {} into {}",
                self.get_type(),
                destination.get_type()
            ));
        }

        // info(format!("{}", index_pointer.get_address()).as_str());

        DynamicFromCopyInstruction::new_alloc(
            memory_managers,
            _super.get_address(),
            self.get_size(),
            index_pointer.get_address(),
            destination.address,
            self.get_size(),
        );

        Ok(())
    }

    /// Sets the value at an index to `source`
    fn set_index(
        &self,
        _super: &Type,
        memory_managers: &mut MemoryManagers,
        index_pointer: &Type,
        source: &Type,
    ) -> Result<(), String> {
        match index_pointer.get_type() {
            TypeSymbol::Pointer => {}
            _ => return Err("Only pointers are supported for indexing this type".to_string()),
        }

        if self.get_type() != source.get_type() {
            return Err(format!(
                "Cannot move value from {} into indexed {}",
                source.get_type(),
                self.get_type()
            ));
        }

        DynamicToCopyInstruction::new_alloc(
            memory_managers,
            _super.get_address(),
            self.get_size(),
            index_pointer.get_address(),
            source.address,
            self.get_size(),
        );

        Ok(())
    }

    /// Gets the `TypeSymbol` corresponding to this `Type`
    fn get_type(&self) -> TypeSymbol;

    /// Gets return type if this `Type` can be called
    fn get_return_type(&self) -> Result<TypeSymbol, String> {
        Err(format!("{} cannot be called", self.get_type()))
    }

    /// Gets the size of this `Type`
    fn get_size(&self) -> usize;

    /// Calls this `Type` if it can be called. Put the return value into `destination` if it is not `None`
    fn call(
        &self,
        _memory_managers: &mut MemoryManagers,
        _arguments: Vec<&Type>,
        _destination: Option<&Type>,
    ) -> Result<(), String> {
        Err(format!("{} cannot be called", self.get_type()))
    }

    /// Gets the `TypeSymbol` that the given operation would return
    fn get_operation_type(
        &self,
        _lhs: &Type,
        operator: &Operator,
        rhs: Option<&Type>,
    ) -> Result<TypeSymbol, String> {
        create_op_not_impl_error(operator, self.get_type(), rhs)
    }

    /// Performs `operator` on `self` and `rhs` (if it is `Some`). Puts result in `destination`
    fn operate(
        &self,
        _lhs: &Type,
        _memory_managers: &mut MemoryManagers,
        operator: Operator,
        rhs: Option<&Type>,
        _destination: &Type,
    ) -> Result<(), String> {
        create_op_not_impl_error(&operator, self.get_type(), rhs)
    }

    /// Clones this type
    fn clone(&self) -> Box<dyn TypeTrait>;
}

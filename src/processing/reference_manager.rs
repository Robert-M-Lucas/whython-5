pub mod class;
pub mod function;

use crate::processing::reference_manager::class::ClassReference;
use crate::processing::reference_manager::function::FunctionReference;
use crate::processing::types::Type;

pub enum ReferenceType {
    Variable(Box<dyn Type>),
    Function(FunctionReference),
    Class(ClassReference),
}

pub struct NamedReference {
    pub name: String,
    pub reference: ReferenceType,
}

impl NamedReference {
    pub fn is_variable(&self) -> bool {
        match self.reference {
            ReferenceType::Variable(_) => true,
            _ => false,
        }
    }

    pub fn new_variable(name: String, variable: Box<dyn Type>) -> Self {
        NamedReference {
            name,
            reference: ReferenceType::Variable(variable),
        }
    }

    pub fn new_function(name: String, function: FunctionReference) -> Self {
        NamedReference {
            name,
            reference: ReferenceType::Function(function),
        }
    }

    pub fn get_variable_ref(&self) -> Result<&Box<dyn Type>, String> {
        match &self.reference {
            ReferenceType::Variable(variable) => Ok(variable),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn get_variable(self) -> Result<Box<dyn Type>, String> {
        match self.reference {
            ReferenceType::Variable(variable) => Ok(variable),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn clone_variable(&self) -> Result<NamedReference, String> {
        match &self.reference {
            ReferenceType::Variable(t) => {
                Ok(NamedReference::new_variable(self.name.clone(), t.duplicate()))
            },
            _ => Err("Reference not a variable".to_string())
        }
    }

    pub fn get_function_ref(&self) -> Result<&FunctionReference, String> {
        match &self.reference {
            ReferenceType::Function(function) => Ok(function),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn get_function_mut(&mut self) -> Result<&mut FunctionReference, String> {
        match &mut self.reference {
            ReferenceType::Function(function) => Ok(function),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn get_function(self) -> Result<FunctionReference, String> {
        match self.reference {
            ReferenceType::Function(function) => Ok(function),
            _ => Err("Reference is not a variable".to_string()),
        }
    }
}

#[derive(Default)]
pub struct ReferenceStack {
    stack: Vec<ReferenceManager>,
    reference_depth_limit: usize,
}

impl ReferenceStack {
    pub fn new() -> Self {
        ReferenceStack {
            stack: Vec::new(),
            reference_depth_limit: 0,
        }
    }

    pub fn get_and_remove_stack_contents(&mut self) -> Vec<NamedReference> {
        let mut last = self.stack.pop().unwrap();
        let r = last.references;
        last.references = Vec::new();
        self.stack.push(last);
        r
    }

    pub fn set_reference_depth_limit(&mut self, reference_depth_limit: usize) {
        self.reference_depth_limit = reference_depth_limit
    }

    pub fn get_reference_depth_limit(&self) -> usize {
        self.reference_depth_limit
    }

    pub fn get_depth(&self) -> usize {
        if self.stack.is_empty() {
            panic!("No reference managers");
        }
        self.stack.len() - 1
    }

    /// Registers a variable
    pub fn register_reference(&mut self, reference: NamedReference) -> Result<(), String> {
        return self.stack.last_mut().unwrap().register_reference(reference);
    }

    /// Registers a variable at a layer `offset` above the current one
    pub fn register_reference_with_offset(
        &mut self,
        reference: NamedReference,
        offset: usize,
    ) -> Result<(), String> {
        let len = self.stack.len();
        self.stack[(len - 1) - offset].register_reference(reference)
    }

    /// Searches for a variable going up the reference stack
    pub fn get_reference(&self, name: &str) -> Result<&NamedReference, String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            if let Ok(r) = &self.stack[i].get_reference(name) {
                if i < self.reference_depth_limit && r.is_variable() {
                    continue;
                }
                return Ok(r);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(format!("Reference '{}' not found", name))
    }

    pub fn get_and_remove_reference(&mut self, name: &str) -> Result<(NamedReference, usize), String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            if let Ok(r) =
                self.stack[i].get_and_remove_reference(name, i < self.reference_depth_limit)
            {
                if i < self.reference_depth_limit && r.is_variable() {
                    continue;
                }
                return Ok((r, self.stack.len() - 1 - i));
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(format!("Reference '{}' not found", name))
    }

    /// Adds a reference handler (adds a variable scope)
    pub fn add_handler(&mut self) {
        self.stack.push(ReferenceManager::new());
    }

    /// Removes a reference handler (removes a variable scope)
    pub fn remove_handler(&mut self) {
        if self.reference_depth_limit >= self.stack.len() {
            panic!("Number of reference stacks lower than reference depth limit!");
        }
        self.stack.pop();
    }
}

#[derive(Default)]
pub struct ReferenceManager {
    references: Vec<NamedReference>, // Type, Array Index
}

impl ReferenceManager {
    pub fn new() -> Self {
        ReferenceManager {
            references: Vec::new(),
        }
    }

    /// Registers a variable
    pub fn register_reference(&mut self, reference: NamedReference) -> Result<(), String> {
        if self.get_reference(reference.name.as_str()).is_ok() {
            return Err(format!(
                "Reference with name '{}' already exists",
                reference.name
            ));
        }
        self.references.push(reference);
        Ok(())
    }

    /// Returns the `Some(variable)` if it exists. If not, returns `None`
    pub fn get_reference(&self, name: &str) -> Result<&NamedReference, String> {
        match self.references.iter().find(|&v| *v.name.as_str() == *name) {
            Some(r) => Ok(r),
            None => Err(format!("Reference '{}' not found", name)),
        }
    }

    pub fn get_and_remove_reference(
        &mut self,
        name: &str,
        disallow_variables: bool,
    ) -> Result<NamedReference, String> {
        for i in 0..self.references.len() {
            if self.references[i].name == *name
                && (!disallow_variables || !self.references[i].is_variable())
            {
                return Ok(self.references.remove(i));
            }
        }

        Err(format!("Reference '{}' not found", name))
    }
}

pub mod class;
pub mod function;

use crate::processing::reference_manager::class::ClassReference;
use crate::processing::reference_manager::function::FunctionReference;
use crate::processing::types::Type;
use crate::util::join_reference_name;

fn cant_find_reference_error(name: &[String], fail_point: usize) -> String {
    let mut error_string = String::from("Searching for reference failed at token: ");

    for i in 0..name.len() {
        if i == fail_point {
            error_string.push('*');
        }
        error_string += name[i].as_str();
        if i == fail_point {
            error_string.push('*');
        }

        if i != name.len() - 1 {
            error_string.push('.');
        }
    }

    error_string
}

pub enum Reference {
    Variable(Box<dyn Type>),
    Function(FunctionReference),
    Class(ClassReference),
}

pub struct ReferenceHandler {
    pub name: String,
    reference: Reference,
    sub_references: Vec<ReferenceHandler>,
}

impl ReferenceHandler {
    pub fn new(reference: Reference, name: String) -> ReferenceHandler {
        Self {
            name,
            reference,
            sub_references: Vec::new(),
        }
    }

    pub fn reference(&self) -> &Reference {
        &self.reference
    }
    pub fn reference_mut(&mut self) -> &mut Reference {
        &mut self.reference
    }

    pub fn add_sub_reference(&mut self, reference: Reference, name: String) -> &ReferenceHandler {
        self.sub_references
            .push(ReferenceHandler::new(reference, name));
        self.sub_references.last().unwrap()
    }

    /// Searches for a reference. If the top level fails, returns `Err(None)`. If a lower level fails, returns `Err([error])`
    pub fn get_reference(
        &self,
        name: &[String],
        depth: usize,
    ) -> Result<&Reference, Option<String>> {
        Ok(self.get_reference_handler(name, depth)?.reference())
    }

    /// Searches for a reference. If the top level fails, returns `Err(None)`. If a lower level fails, returns `Err([error])`
    pub fn get_reference_mut(
        &mut self,
        name: &[String],
        depth: usize,
    ) -> Result<&mut Reference, Option<String>> {
        Ok(self.get_reference_handler_mut(name, depth)?.reference_mut())
    }

    pub fn get_reference_handler(
        &self,
        name: &[String],
        depth: usize,
    ) -> Result<&ReferenceHandler, Option<String>> {
        if name[depth] != self.name {
            return Err(None);
        }

        if name.len() - 1 == depth {
            return Ok(self);
        }

        if depth + 1 != name.len() {
            for sub_reference in &self.sub_references {
                let result = sub_reference.get_reference_handler(name, depth + 1);
                match result {
                    Ok(_) | Err(Some(_)) => {
                        return result;
                    }
                    Err(None) => {}
                };
            }
        }

        Err(Some(cant_find_reference_error(name, depth + 1)))
    }

    pub fn get_reference_handler_mut(
        &mut self,
        name: &[String],
        depth: usize,
    ) -> Result<&mut ReferenceHandler, Option<String>> {
        if name[depth] != self.name {
            return Err(None);
        }

        if name.len() - 1 == depth {
            return Ok(self);
        }

        if depth + 1 != name.len() {
            for sub_reference in &mut self.sub_references {
                let result = sub_reference.get_reference_handler_mut(name, depth + 1);
                match result {
                    Ok(_) | Err(Some(_)) => {
                        return result;
                    }
                    Err(None) => {}
                };
            }
        }

        Err(Some(cant_find_reference_error(name, depth + 1)))
    }
}

impl Reference {
    pub fn is_variable(&self) -> bool {
        matches!(self, Reference::Variable(_))
    }

    pub fn get_variable_ref(&self) -> Result<&dyn Type, String> {
        match &self {
            Reference::Variable(variable) => Ok(variable.as_ref()),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn get_variable(self) -> Result<Box<dyn Type>, String> {
        match self {
            Reference::Variable(variable) => Ok(variable),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn clone_variable(&self) -> Result<Reference, String> {
        match &self {
            Reference::Variable(t) => Ok(Reference::Variable(t.duplicate())),
            _ => Err("Reference not a variable".to_string()),
        }
    }

    pub fn get_function_ref(&self) -> Result<&FunctionReference, String> {
        match &self {
            Reference::Function(function) => Ok(function),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn get_function_mut(&mut self) -> Result<&mut FunctionReference, String> {
        match self {
            Reference::Function(function) => Ok(function),
            _ => Err("Reference is not a variable".to_string()),
        }
    }

    pub fn get_function(self) -> Result<FunctionReference, String> {
        match self {
            Reference::Function(function) => Ok(function),
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

    pub fn get_top_stack(&self) -> &Vec<ReferenceHandler> {
        &self.stack.last().unwrap().references
    }

    // pub fn get_and_remove_stack_contents(&mut self) -> Vec<ReferenceHandler> {
    //     let mut last = self.stack.pop().unwrap();
    //     let r = last.references;
    //     last.references = Vec::new();
    //     self.stack.push(last);
    //     r
    // }

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
    pub fn register_reference(
        &mut self,
        reference: Reference,
        name: Vec<String>,
    ) -> Result<(), String> {
        if name.len() == 1 {
            self.stack
                .last_mut()
                .unwrap()
                .register_reference(reference, name)
        } else {
            let handler = self.get_reference_handler_mut(&name[..(name.len() - 1)])?;
            handler.add_sub_reference(reference, name.into_iter().last().unwrap());
            Ok(())
        }
    }

    /// Registers a variable at a layer `offset` above the current one
    pub fn register_reference_with_offset(
        &mut self,
        reference: Reference,
        name: Vec<String>,
        offset: usize,
    ) -> Result<(), String> {
        let len = self.stack.len();
        self.stack[(len - 1) - offset].register_reference(reference, name)
    }

    /// Searches for a variable going up the reference stack
    pub fn get_reference(&self, name: &[String]) -> Result<&Reference, String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            match self.stack[i].get_reference(name) {
                Ok(Some(r)) => {
                    if i < self.reference_depth_limit && r.is_variable() {
                        continue;
                    }
                    return Ok(r);
                }
                Err(e) => return Err(e),
                Ok(None) => {}
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(cant_find_reference_error(name, 0))
    }

    pub fn get_reference_and_offset(&self, name: &[String]) -> Result<(&Reference, usize), String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            match self.stack[i].get_reference(name) {
                Ok(Some(r)) => {
                    if i < self.reference_depth_limit && r.is_variable() {
                        continue;
                    }
                    return Ok((r, self.stack.len() - 1 - i));
                }
                Err(e) => return Err(e),
                Ok(None) => {}
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(cant_find_reference_error(name, 0))
    }

    /// Searches for a variable going up the reference stack
    pub fn get_reference_mut(&mut self, name: &[String]) -> Result<&mut Reference, String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            match self.stack[i].get_reference(name) {
                Ok(Some(r)) => {
                    if i < self.reference_depth_limit && r.is_variable() {
                        continue;
                    }
                    // TODO
                    //? Redundant function call to appease borrow checkers
                    let r = self.stack[i].get_reference_mut(name).unwrap().unwrap();
                    return Ok(r);
                }
                Err(e) => return Err(e),
                Ok(None) => {}
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(cant_find_reference_error(name, 0))
    }

    pub fn get_reference_handler(&self, name: &[String]) -> Result<&ReferenceHandler, String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            match self.stack[i].get_reference_handler(name) {
                Ok(Some(r)) => {
                    if i < self.reference_depth_limit && r.reference.is_variable() {
                        continue;
                    }
                    return Ok(r);
                }
                Err(e) => return Err(e),
                Ok(None) => {}
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(cant_find_reference_error(name, 0))
    }

    /// Searches for a variable going up the reference stack
    pub fn get_reference_handler_mut(
        &mut self,
        name: &[String],
    ) -> Result<&mut ReferenceHandler, String> {
        //? Go up the stack and search for a variable

        let mut i = self.stack.len() - 1;
        loop {
            match self.stack[i].get_reference_handler_mut(name) {
                Ok(Some(r)) => {
                    if i < self.reference_depth_limit && r.reference.is_variable() {
                        continue;
                    }
                    // TODO
                    //? Redundant function call to appease borrow checkers
                    let r = self.stack[i]
                        .get_reference_handler_mut(name)
                        .unwrap()
                        .unwrap();
                    return Ok(r);
                }
                Err(e) => return Err(e),
                Ok(None) => {}
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        Err(cant_find_reference_error(name, 0))
    }

    // pub fn get_and_remove_reference(&mut self, name: &[String]) -> Result<(Reference, usize), String> {
    //     //? Go up the stack and search for a variable
    //
    //     let mut i = self.stack.len() - 1;
    //     loop {
    //         if let Ok(r) =
    //             self.stack[i].get_and_remove_reference(name, i < self.reference_depth_limit)
    //         {
    //             if i < self.reference_depth_limit && r.is_variable() {
    //                 continue;
    //             }
    //             return Ok((r, self.stack.len() - 1 - i));
    //         }
    //         if i == 0 {
    //             break;
    //         }
    //         i -= 1;
    //     }
    //
    //     Err(cant_find_reference_error(name, 0))
    // }

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
    references: Vec<ReferenceHandler>, // Type, Array Index
}

impl ReferenceManager {
    pub fn new() -> Self {
        ReferenceManager {
            references: Vec::new(),
        }
    }

    /// Registers a variable
    pub fn register_reference(
        &mut self,
        reference: Reference,
        name: Vec<String>,
    ) -> Result<(), String> {
        if matches!(self.get_reference(&name), Ok(Some(_))) {
            return Err(format!(
                "Reference with name '{}' already exists",
                join_reference_name(&name)
            ));
        }

        if name.len() == 1 {
            let name = name.into_iter().next().unwrap();
            self.references.push(ReferenceHandler::new(reference, name));
        } else {
            let handler = self.get_reference_handler_mut(&name[..(name.len() - 1)])?;
            if let Some(handler) = handler {
                handler.add_sub_reference(reference, name.into_iter().last().unwrap());
            } else {
                return Err(cant_find_reference_error(&name, 0));
            }
        }
        Ok(())
    }

    /// Returns the `Some(variable)` if it exists. If not, returns `None`
    pub fn get_reference(&self, name: &[String]) -> Result<Option<&Reference>, String> {
        for reference in &self.references {
            match reference.get_reference(name, 0) {
                Ok(reference) => return Ok(Some(reference)),
                Err(Some(error)) => return Err(error),
                Err(None) => {}
            };
        }

        Ok(None)
    }

    /// Returns the `Some(variable)` if it exists. If not, returns `None`
    pub fn get_reference_mut(&mut self, name: &[String]) -> Result<Option<&mut Reference>, String> {
        for reference in &mut self.references {
            match reference.get_reference_mut(name, 0) {
                Ok(reference) => return Ok(Some(reference)),
                Err(Some(error)) => return Err(error),
                Err(None) => {}
            };
        }

        Ok(None)
    }

    /// Returns the `Some(variable)` if it exists. If not, returns `None`
    pub fn get_reference_handler(
        &self,
        name: &[String],
    ) -> Result<Option<&ReferenceHandler>, String> {
        for reference in &self.references {
            match reference.get_reference_handler(name, 0) {
                Ok(reference) => return Ok(Some(reference)),
                Err(Some(error)) => return Err(error),
                Err(None) => {}
            };
        }

        Ok(None)
    }

    /// Returns the `Some(variable)` if it exists. If not, returns `None`
    pub fn get_reference_handler_mut(
        &mut self,
        name: &[String],
    ) -> Result<Option<&mut ReferenceHandler>, String> {
        for reference in &mut self.references {
            match reference.get_reference_handler_mut(name, 0) {
                Ok(reference) => return Ok(Some(reference)),
                Err(Some(error)) => return Err(error),
                Err(None) => {}
            };
        }

        Ok(None)
    }

    // pub fn get_and_remove_reference(
    //     &mut self,
    //     name: &[String],
    //     disallow_variables: bool,
    // ) -> Result<Reference, String> {
    //
    //     for i in 0..self.references.len() {
    //         match reference[i].get_reference(name, 0) {
    //             Ok(reference) => return Ok(reference),
    //             Err(Some(error)) => return Err(error),
    //             Err(None) => {}
    //         };
    //     }
    //
    //     Err(cant_find_reference_error(name, 0))
    // }
}

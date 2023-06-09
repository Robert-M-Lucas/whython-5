use crate::processing::types::Type;

pub enum ReferenceType {
    Variable(Box<dyn Type>),
    Function,
    Class,
}

pub struct NamedReference {
    pub name: String,
    pub reference: ReferenceType,
}

impl NamedReference {
    pub fn new_variable(name: String, variable: Box<dyn Type>) -> Self {
        NamedReference {
            name,
            reference: ReferenceType::Variable(variable),
        }
    }

    pub fn get_variable(&self) -> Result<&Box<dyn Type>, String> {
        match &self.reference {
            ReferenceType::Function | ReferenceType::Class => {
                Err("Reference is not a variable".to_string())
            }
            ReferenceType::Variable(variable) => Ok(variable),
        }
    }
}

#[derive(Default)]
pub struct ReferenceStack {
    stack: Vec<ReferenceManager>,
}

impl ReferenceStack {
    pub fn new() -> Self {
        ReferenceStack {
            stack: vec![ReferenceManager::new()],
        }
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
        let mut reference_manager = &self.stack[i];
        loop {
            let r = reference_manager.get_reference(name);
            if let Ok(i) = r {
                return Ok(i);
            }
            if i == 0 {
                break;
            }
            i -= 1;
            reference_manager = &self.stack[i];
        }

        Err(format!("Reference '{}' not found", name))
    }

    /// Adds a reference handler (adds a variable scope)
    pub fn add_handler(&mut self) {
        self.stack.push(ReferenceManager::new());
    }

    /// Removes a reference handler (removes a variable scope)
    pub fn remove_handler(&mut self) {
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
}

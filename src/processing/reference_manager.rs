use crate::processing::types::Type;

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
    pub fn register_variable(&mut self, variable: Type, name: String) -> Result<(), String> {
        return self
            .stack
            .last_mut()
            .unwrap()
            .register_variable(variable, name);
    }

    /// Registers a variable at a layer `offset` above the current one
    pub fn register_variable_with_offset(
        &mut self,
        variable: Type,
        name: String,
        offset: usize,
    ) -> Result<(), String> {
        let len = self.stack.len();
        self.stack[(len - 1) - offset].register_variable(variable, name)
    }

    /// Searches for a variable going up the reference stack
    pub fn get_variable(&self, name: &str) -> Result<&Type, String> {
        //? Go up the stack and search for a variable
        let mut i = self.stack.len() - 1;
        let mut reference_manager = &self.stack[i];
        loop {
            let r = reference_manager.get_variable(name);
            if let Some(i) = r {
                return Ok(i);
            }
            if i == 0 {
                break;
            }
            i -= 1;
            reference_manager = &self.stack[i];
        }

        Err(format!("Variable '{}' not found", name))
    }

    /// Adds a reference handler (adds a variable scope)
    pub fn add_handler(&mut self) {
        self.stack.push(ReferenceManager::new());
    }

    /// Removes a reference handler (removes a variable scope)
    pub fn remove_handler(&mut self) {
        self.stack.pop();
    }

    /*    pub fn start_handler_remove(&mut self) { self.stack_removed = Some(self.stack.pop().unwrap()); }

    pub fn cancel_handler_remove(&mut self) {
        self.stack.push(self.stack_removed.unwrap());
        self.stack_removed = None;
    }

    pub fn complete_handler_removal(&mut self) { self.stack_removed = None; }*/
}

#[derive(Default)]
pub struct ReferenceManager {
    variables: Vec<Type>, // Type, Array Index
}

impl ReferenceManager {
    pub fn new() -> Self {
        ReferenceManager {
            variables: Vec::new(),
        }
    }

    /// Registers a variable
    pub fn register_variable(&mut self, mut variable: Type, name: String) -> Result<(), String> {
        if self.get_variable(name.as_str()).is_some() {
            return Err(format!("Variable with name '{}' already exists", name));
        }
        variable.set_name(name);
        self.variables.push(variable);
        Ok(())
    }

    /// Returns the `Some(variable)` if it exists. If not, returns `None`
    pub fn get_variable(&self, name: &str) -> Option<&Type> {
        self.variables.iter().find(|&v| *v.get_name() == *name)
    }
}

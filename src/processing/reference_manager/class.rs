use crate::processing::reference_manager::function::FunctionReference;
use crate::processing::types::Type;

pub struct ClassReference {
    properties: Vec<(String, Box<dyn Type>)>,
    functions: Vec<(String, FunctionReference)>,
}

impl ClassReference {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            functions: Vec::new(),
        }
    }
}

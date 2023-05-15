use crate::processing::symbols::{Operator, TypeSymbol};

pub trait UninstantiatedType {
    fn new(&self) -> Box<dyn Type>;
}

pub trait Type {
    fn get_type_symbol(&self) -> TypeSymbol;
}

pub trait Operation<LHS> {
    fn get_symbol(&self) -> Operator;

    fn get_result_type(&self, rhs: Option<TypeSymbol>) -> Option<TypeSymbol>;

    fn operate(&self, lhs: &LHS, rhs: Box<dyn Type>) -> Result<(), String>;
}

pub struct BoolWrapper {}

impl UninstantiatedType for BoolWrapper {
    fn new(&self) -> Box<dyn Type> {
        Box::new(BoolType::new())
    }
}

pub struct BoolType {
    operators: Vec<Box<dyn Operation<BoolType>>>
}

impl BoolType {
    pub fn new() -> Self {
        Self {
            operators: vec![
                Box::new(BoolAnd{})
            ]
        }
    }

    pub fn operate(&self, rhs: Box<dyn Type>) -> Result<(), String> {
        for operator in self.operators.iter() {
            if operator.get_result_type(Some(rhs.get_type_symbol())).is_some() {
                return operator.operate(self, rhs);
            }
        }

        Err("Operations not found!".to_string())
    }
}

impl Type for BoolType {
    fn get_type_symbol(&self) -> TypeSymbol {
        TypeSymbol::Boolean
    }
}

pub struct BoolAnd {}

impl Operation<BoolType> for BoolAnd {
    fn get_symbol(&self) -> Operator {
        Operator::And
    }

    fn get_result_type(&self, rhs: Option<TypeSymbol>) -> Option<TypeSymbol> {
        let Some(rhs) = rhs else { return None; };
        match rhs {
            TypeSymbol::Boolean => Some(TypeSymbol::Boolean),
            _ => None
        }
    }

    fn operate(&self, lhs: &BoolType, rhs: Box<dyn Type>) -> Result<(), String> {
        todo!()
    }
}
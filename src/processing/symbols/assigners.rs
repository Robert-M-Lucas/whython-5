use super::Operator;
use super::Symbol;
use super::SymbolHandler;
use crate::processing::symbols::Symbol::ArithmeticBlock;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum Assigner {
    Setter,
    AdditionSetter,
    SubtractionSetter,
    ProductSetter,
    DivisionSetter,
}

impl Assigner {
    pub fn get_expanded_equivalent(&self, lhs: Symbol, rhs: Vec<Symbol>) -> Vec<Symbol> {
        let equivalent = match self {
            Assigner::Setter => {
                return rhs;
            }
            Assigner::AdditionSetter => Operator::Add,
            Assigner::SubtractionSetter => Operator::Subtract,
            Assigner::ProductSetter => Operator::Product,
            Assigner::DivisionSetter => Operator::Divide,
        };

        vec![lhs, Symbol::Operator(equivalent), ArithmeticBlock(rhs)]
    }
}

pub struct AssignerSymbolHandler {}

impl SymbolHandler for AssignerSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        match string {
            "=" => Some(Symbol::Assigner(Assigner::Setter)),
            "+=" => Some(Symbol::Assigner(Assigner::AdditionSetter)),
            "-=" => Some(Symbol::Assigner(Assigner::SubtractionSetter)),
            "*=" => Some(Symbol::Assigner(Assigner::ProductSetter)),
            "/=" => Some(Symbol::Assigner(Assigner::DivisionSetter)),
            _ => None,
        }
    }
}

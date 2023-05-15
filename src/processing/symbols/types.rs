use crate::processing::symbols::{Symbol, SymbolHandler};

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
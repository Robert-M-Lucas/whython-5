use crate::processing::symbols::{Symbol, SymbolHandler};

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum TypeSymbol {
    Integer,
    Boolean,
    Character,
    Function,
    Pointer,
}

// TODO: Copy this implementation patterns
pub struct TypeSymbolHandler {}

impl TypeSymbolHandler {
    pub fn get_raw_symbol(string: &str) -> Option<TypeSymbol> {
        match string {
            "int" => Some(TypeSymbol::Integer),
            "bool" => Some(TypeSymbol::Boolean),
            "char" => Some(TypeSymbol::Character),
            "ptr" => Some(TypeSymbol::Pointer),
            _ => None,
        }
    }
}

impl SymbolHandler for TypeSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        Some(Symbol::Type(TypeSymbolHandler::get_raw_symbol(string)?))
    }
}

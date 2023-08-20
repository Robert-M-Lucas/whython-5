use crate::processing::symbols::{Symbol, SymbolHandler};

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug, strum_macros::EnumIter)]
pub enum TypeSymbol {
    Integer,
    Boolean,
    Character,
    // Function,
    Pointer,
}

impl TypeSymbol {
    pub fn get_code_representation(&self) -> &str {
        match self {
            TypeSymbol::Integer => "int",
            TypeSymbol::Boolean => "bool",
            TypeSymbol::Character => "char",
            TypeSymbol::Pointer => "ptr",
        }
    }
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
    fn get_symbol(string: &str) -> Result<Option<Symbol>, String> {
        Ok(TypeSymbolHandler::get_raw_symbol(string).and_then(|s| Some(Symbol::Type(s))))
    }
}

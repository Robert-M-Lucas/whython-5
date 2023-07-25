use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum Keyword {
    Break,
    Continue,
    Dump,
    ViewMemory,
    As,
}

pub struct KeywordSymbolHandler {}

impl Keyword {
    pub fn get_code_representation(&self) -> &str {
        match self {
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::Dump => "dump",
            Keyword::ViewMemory => "viewmem",
            Keyword::As => "as",
        }
    }
}

impl SymbolHandler for KeywordSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        match string {
            "break" => Some(Symbol::Keyword(Keyword::Break)),
            "continue" => Some(Symbol::Keyword(Keyword::Continue)),
            "dump" => Some(Symbol::Keyword(Keyword::Dump)),
            "viewmem" => Some(Symbol::Keyword(Keyword::ViewMemory)),
            "as" => Some(Symbol::Keyword(Keyword::As)),
            _ => None,
        }
    }
}

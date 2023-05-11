use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display)]
pub enum Keyword {
    Break,
    Continue,
}

pub struct KeywordSymbolHandler {}

impl SymbolHandler for KeywordSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        match string {
            "break" => Some(Symbol::Keyword(Keyword::Break)),
            "continue" => Some(Symbol::Keyword(Keyword::Continue)),
            _ => None,
        }
    }
}

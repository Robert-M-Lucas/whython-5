use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum Punctuation {
    ListSeparator,
}

pub struct PunctuationSymbolHandler {}

impl SymbolHandler for PunctuationSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        match string {
            "," => Some(Symbol::Punctuation(Punctuation::ListSeparator)),
            _ => None,
        }
    }
}

// impl Punctuation {
//     pub(crate) fn get_name(&self) -> &str {
//         return match self {
//             Punctuation::ListSeparator => "ListSeparator"
//         }
//     }
// }

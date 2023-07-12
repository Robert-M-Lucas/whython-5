use super::Symbol;
use super::SymbolHandler;

#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug)]
pub enum Block {
    While,
    Loop,
    If,
    Elif,
    Else,
    Function,
    BaseBlock,
}

pub struct BlockSymbolHandler {}

impl SymbolHandler for BlockSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        match string {
            "while" => Some(Symbol::Block(Block::While)),
            "loop" => Some(Symbol::Block(Block::Loop)),
            "if" => Some(Symbol::Block(Block::If)),
            "elif" => Some(Symbol::Block(Block::Elif)),
            "else" => Some(Symbol::Block(Block::Else)),
            "fn" => Some(Symbol::Block(Block::Function)),
            "block" => Some(Symbol::Block(Block::BaseBlock)),
            _ => None,
        }
    }
}

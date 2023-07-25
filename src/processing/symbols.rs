mod assigners;
mod blocks;
mod builtins;
mod keywords;
mod literals;
mod operators;
mod punctuation;
mod types;

pub use assigners::Assigner;
use assigners::AssignerSymbolHandler;

pub use literals::Literal;
use literals::LiteralSymbolHandler;
pub use literals::STRING_DELIMITERS;

pub use operators::Operator;
use operators::OperatorSymbolHandler;

pub use types::TypeSymbol;
use types::TypeSymbolHandler;

pub use blocks::Block;
use blocks::BlockSymbolHandler;

pub use builtins::Builtin;
use builtins::BuiltinSymbolHandler;

pub use punctuation::Punctuation;
pub use punctuation::PunctuationSymbolHandler;
pub use punctuation::LIST_SEPARATOR_CHARACTER;

pub use keywords::Keyword;
pub use keywords::KeywordSymbolHandler;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum Symbol {
    Assigner(Assigner),
    Literal(Literal),
    Operator(Operator),
    BracketedSection(Vec<Symbol>),
    Indexer(Box<Symbol>, Vec<Symbol>),
    List(Vec<Vec<Symbol>>),
    MethodCall(Box<Symbol>, String, Vec<Vec<Symbol>>), // ? Value, method, arguments
    Type(TypeSymbol),
    Block(Block),
    Builtin(Builtin),
    Punctuation(Punctuation),
    Name(String),
    Keyword(Keyword),
}

pub trait SymbolHandler {
    /// Converts a string to a symbol. Returns `None` if no symbol matches the string
    fn get_symbol(string: &str) -> Option<Symbol>;
}

/// Converts a string to a symbol. Returns `None` if no symbol matches the string
pub fn get_all_symbol(string: &str) -> Option<Symbol> {
    AllSymbolHandler::get_symbol(string)
}

/// Converts an arithmetic block into a `Literal::ParameterList(parameters)`
pub fn try_bracketed_into_parameters(bracketed: &Symbol) -> Result<Literal, String> {
    fn formatting_error() -> String {
        "Parameters must be formatted ([Type] [Name] , [Type] [Name] , [...])".to_string()
    }

    let list = match bracketed {
        Symbol::BracketedSection(list) => list,
        _ => panic!("Must be bracketed section"),
    };

    if list.is_empty() {
        return Ok(Literal::ParameterList(Vec::new()));
    }

    let mut parameter_list: Vec<(TypeSymbol, String)> = Vec::new();

    let mut i: usize = 0;

    while i < list.len() {
        // Type without name
        if list.len() - i == 1 {
            return Err(formatting_error());
        }

        // No type
        let type_symbol = match list[i] {
            Symbol::Type(type_symbol) => type_symbol,
            _ => return Err(formatting_error()),
        };

        // No name
        let name = match &list[i + 1] {
            Symbol::Name(name) => name.clone(),
            _ => return Err(formatting_error()),
        };

        // Check for list separator
        if i + 2 < list.len() {
            match list[i + 2] {
                Symbol::Punctuation(Punctuation::ListSeparator) => (), // =>
                // {
                //     #[allow(unreachable_patterns)]
                //     match punctuation {
                //         Punctuation::ListSeparator => (),
                //         _ => return Err(formatting_error()),
                //     }
                // }
                _ => return Err(formatting_error()),
            }
        }

        parameter_list.push((type_symbol, name));

        i += 3;
    }

    Ok(Literal::ParameterList(parameter_list))
}

const ALLOWED_CHARS_IN_NAME: &str = "abcdefghijklmnopqrstuvwxyz_";

struct AllSymbolHandler {}

impl SymbolHandler for AllSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        AssignerSymbolHandler::get_symbol(string)
            .or_else(|| OperatorSymbolHandler::get_symbol(string))
            .or_else(|| TypeSymbolHandler::get_symbol(string))
            .or_else(|| BlockSymbolHandler::get_symbol(string))
            .or_else(|| BuiltinSymbolHandler::get_symbol(string))
            .or_else(|| LiteralSymbolHandler::get_symbol(string))
            .or_else(|| PunctuationSymbolHandler::get_symbol(string))
            .or_else(|| KeywordSymbolHandler::get_symbol(string))
            .or_else(|| {
                for c in string.chars() {
                    if !ALLOWED_CHARS_IN_NAME.contains(c) {
                        return None;
                    }
                }

                Some(Symbol::Name(String::from(string)))
            })
    }
}

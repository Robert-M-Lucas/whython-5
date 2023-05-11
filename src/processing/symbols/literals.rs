use super::Symbol;
use super::SymbolHandler;
use crate::processing::types::TypeSymbol;

#[derive(PartialEq, Clone, strum_macros::Display)]
pub enum Literal {
    String(String),
    Int(i64),
    Bool(bool),
    ParameterList(Vec<(TypeSymbol, String)>),
    None,
}

pub struct LiteralSymbolHandler {}

pub const STRING_DELIMITERS: [char; 2] = ['\'', '"'];

const ESCAPE_CODES: [(char, char); 3] = [('n', '\n'), ('\\', '\\'), ('0', '\0')];

/// Takes an input string and replaces escape codes with their corresponding values
fn format_escape_codes(input: String) -> String {
    let mut output = String::new();
    let mut next = false;
    'char_loop: for c in input.chars() {
        if next {
            next = false;
            for code in ESCAPE_CODES {
                if c == code.0 {
                    output.push(code.1);
                    continue 'char_loop;
                }
            }
        }

        if c == '\\' && !next {
            next = true;
        } else {
            output.push(c);
        }
    }
    output
}

impl SymbolHandler for LiteralSymbolHandler {
    fn get_symbol(string: &str) -> Option<Symbol> {
        (match string {
            // Boolean
            "true" => Some(Symbol::Literal(Literal::Bool(true))),
            "false" => Some(Symbol::Literal(Literal::Bool(false))),
            "none" => Some(Symbol::Literal(Literal::None)),
            _ => None,
        })
        .or_else(
            // String
            || {
                if string.len() >= 2
                    && STRING_DELIMITERS.contains(&string.chars().next().unwrap())
                    && string.chars().last().unwrap() == string.chars().next().unwrap()
                {
                    return Some(Symbol::Literal(Literal::String(format_escape_codes(
                        string[1..string.len() - 1].to_string(),
                    ))));
                }
                None
            },
        )
        .or_else(
            // Integer
            || match string.parse::<i64>() {
                Ok(ok) => Some(Symbol::Literal(Literal::Int(ok))),
                Err(_) => None,
            },
        )
    }
}

// impl Literal {
//     pub(crate) fn get_name(&self) -> &str {
//         return match self {
//             Literal::StringLiteral(_) => "StringLiteral",
//             Literal::IntLiteral(_) => "IntLiteral",
//             Literal::BoolLiteral(_) => "BoolLiteral",
//             Literal::ParameterList(_) => "ParameterList",
//             Literal::None => "None",
//         }
//     }
// }

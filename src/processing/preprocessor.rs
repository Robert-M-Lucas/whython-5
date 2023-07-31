use crate::bx;
use crate::errors::create_line_error;
use crate::processing::symbols::{get_all_symbol, Punctuation, Symbol, STRING_DELIMITERS, LIST_SEPARATOR_CHARACTER};

pub const COMMENT_CHARACTER: char = '#';
pub const OPEN_BRACKET_CHARACTER: char = '(';
pub const CLOSE_BRACKET_CHARACTER: char = ')';
pub const OPEN_INDEXER_CHARACTER: char = '[';
pub const CLOSE_INDEXER_CHARACTER: char = ']';

/// Takes a line of code and returns an array of symbols
#[allow(clippy::single_match)]
pub fn get_symbols_from_line(line: &str) -> Result<Vec<Symbol>, String> {
    fn process_buffer(buffer: &mut String, symbol_line: &mut Vec<Symbol>) -> Result<(), String> {
        if buffer.is_empty() {
            return Ok(());
        }

        if let Some(symbol) = get_all_symbol(buffer) {
            symbol_line.push(symbol);
            buffer.clear();
            Ok(())
        } else {
            Err(format!("Symbol '{}' not found", buffer))
        }
    }

    let mut symbol_line = Vec::new();

    let mut buffer = String::new();
    let mut in_string: Option<char> = None; // Option<delimiter>
    let mut bracket_depth = 0;
    let mut indexer_depth = 0;
    let mut next_character_escaped = false;

    for c in line.chars() {
        //? String handling
        if let Some(delimiter) = in_string {
            if next_character_escaped {
                buffer.push(c);
                next_character_escaped = false;
                continue;
            }

            if delimiter == c {
                buffer.push(c);
                in_string = None;
                process_buffer(&mut buffer, &mut symbol_line)?;
                continue;
            }

            buffer.push(c);
            continue;
        }
        else if STRING_DELIMITERS.contains(&c) {
            buffer.push(c.clone());
            in_string = Some(c);
            continue;
        }

        //? Comments
        if c == COMMENT_CHARACTER { break; }


        if bracket_depth == 0 && indexer_depth == 0 {
            match c {
                //? Process buffer, ignore c
                ' ' => {
                    process_buffer(&mut buffer, &mut symbol_line)?;
                    continue;
                },
                //? Process buffer, then process c
                OPEN_BRACKET_CHARACTER | CLOSE_BRACKET_CHARACTER | OPEN_INDEXER_CHARACTER | CLOSE_INDEXER_CHARACTER | LIST_SEPARATOR_CHARACTER => {
                    process_buffer(&mut buffer, &mut symbol_line)?;
                }
                _ => {},
            };
        }

        //? List separator
        // if c == LIST_SEPARATOR_CHARACTER {
        //
        // }

        //? Start bracket
        if c == OPEN_BRACKET_CHARACTER {
            if bracket_depth != 0 {
                buffer.push(c);
            }
            bracket_depth += 1;
            continue;
        }

        //? Start bracket
        if c == OPEN_INDEXER_CHARACTER {
            if indexer_depth != 0 {
                buffer.push(c);
            }
            indexer_depth += 1;
            continue;
        }

        //? End bracket section
        if c == CLOSE_BRACKET_CHARACTER {
            bracket_depth -= 1;

            match bracket_depth {
                0 => {
                    symbol_line.push(get_bracketed_symbols_type(get_symbols_from_line(buffer.as_str())?));
                    buffer.clear();
                }
                i32::MIN..=-1 => {
                    return Err(
                        "Closing bracket found with no corresponding opening bracket".to_string(),
                    );
                }
                _ => {
                    buffer.push(c);
                }
            }

            continue;
        }

        //? End indexer
        if c == CLOSE_INDEXER_CHARACTER {
            indexer_depth -= 1;

            match indexer_depth {
                0 => {
                    if symbol_line.len() == 0 {
                        return Err("Indexers must be applied to something".to_string());
                    }

                    let applied_to = symbol_line.pop().unwrap();
                    let index = get_symbols_from_line(buffer.as_str())?;

                    symbol_line.push(Symbol::Indexer(bx!(applied_to), index));

                    buffer.clear();
                }
                i32::MIN..=-1 => {
                    return Err(
                        "Closing indexing bracket found with no corresponding opening bracket".to_string(),
                    );
                }
                _ => {
                    buffer.push(c);
                }
            }

            continue;
        }

        buffer.push(c);
    }

    if in_string.is_some() {
        return Err("Unclosed string".to_string());
    }

    if bracket_depth != 0 {
        return Err("Unclosed brackets".to_string());
    }

    //? Push remaining data
    if !buffer.is_empty() {
        process_buffer(&mut buffer, &mut symbol_line)?;
    }

    Ok(symbol_line)
}

/// Takes code as an input
///
/// Returns `Vec<indentation, symbol line>`
pub fn convert_to_symbols(data: String) -> Result<Vec<(usize, Vec<Symbol>)>, String> {
    let mut output = Vec::new();

    for (line_index, line) in data.lines().enumerate() {
        //? Count indentation
        let mut indentation_count: usize = 0;
        let mut indentation_char_count: usize = 0;
        for c in line.chars() {
            if c == ' ' {
                indentation_count += 1
            } else if c == '\t' {
                indentation_count += 4
            } else {
                break;
            }
            indentation_char_count += 1;
        }
        if indentation_count % 4 != 0 {
            return create_line_error(
                "Indentation must be a multiple of 4 spaces or single tabs".to_string(),
                line_index + 1,
            );
        }

        //? Get symbols
        let symbols = match get_symbols_from_line(&line[indentation_char_count..]) {
            Err(e) => return create_line_error(e, line_index),
            Ok(symbols) => symbols,
        };
        output.push((indentation_count / 4, symbols));
    }

    Ok(output)
}

fn get_bracketed_symbols_type(symbols: Vec<Symbol>) -> Symbol {
    if symbols.is_empty() {
        return Symbol::List(Vec::new());
    }

    let mut has_separator = false;
    for s in &symbols {
        if matches!(s, Symbol::Punctuation(Punctuation::ListSeparator)) {
            has_separator = true;
            break;
        }
    }

    if !has_separator {
        return Symbol::BracketedSection(symbols);
    }

    let mut list = Vec::new();
    let mut item = Vec::new();

    for s in symbols {
        if matches!(s, Symbol::Punctuation(Punctuation::ListSeparator)) {
            list.push(item);
            item = Vec::new();
        } else {
            item.push(s);
        }
    }

    if !item.is_empty() {
        list.push(item);
    }

    Symbol::List(list)
}

use crate::errors::create_line_error;
use crate::processing::symbols::Symbol::ArithmeticBlock;
use crate::processing::symbols::{get_all_symbol, Symbol, STRING_DELIMITERS};
use debugless_unwrap::DebuglessUnwrapErr;

/// Takes a line of code and returns an array of symbols
#[allow(clippy::single_match)]
pub fn get_symbols_from_line(line: &str) -> Result<Vec<Symbol>, String> {
    let mut symbol_line = Vec::new();

    let mut buffer = String::new();
    let mut in_string = false;
    let mut bracket_depth = 0;
    let mut in_indexer = false;
    let mut indexing_start: usize = 0;

    fn process_buffer(buffer: &mut String, symbol_line: &mut Vec<Symbol>) -> Result<(), String> {
        if buffer.is_empty() {
            return Ok(());
        }

        return if let Some(symbol) = get_all_symbol(buffer) {
            symbol_line.push(symbol);
            buffer.clear();
            Ok(())
        } else {
            Err(format!("Symbol '{}' not found", buffer))
        }
    }

    for c in line.chars() {
        //? Comments
        if c == '#' && !in_string {
            break;
        }

        if bracket_depth == 0 && !in_string {
            //? Process buffer and ignore c
            match match c {
                ' ' => Some(process_buffer(&mut buffer, &mut symbol_line)),
                _ => None,
            } {
                Some(value) => match value {
                    Err(e) => return Err(e),
                    Ok(_) => continue,
                },
                None => {}
            }

            //? Process buffer and then treat c normally
            match match c {
                '(' => Some(process_buffer(&mut buffer, &mut symbol_line)),
                _ => None,
            } {
                Some(value) => value?,
                None => {}
            }

            // //? If buffer is empty, process character alone
            // match
            //     match c {
            //         '!' => {
            //             if buffer.len() != 0 { None }
            //             else {
            //                 buffer.push(c);
            //                 Some(process_buffer(&mut buffer, &mut symbol_line))
            //             }
            //         },
            //         _ => None
            //     }
            // {
            //     Some(value) => match value {
            //         Err(e) => return Err(e),
            //         Ok(_) => continue
            //     },
            //     None => {}
            // }

            //? Process character alone
            match match c {
                ',' => {
                    let r = process_buffer(&mut buffer, &mut symbol_line);
                    if r.is_err() {
                        return Err(r.debugless_unwrap_err());
                    }
                    buffer.push(c);
                    Some(process_buffer(&mut buffer, &mut symbol_line))
                }
                _ => None,
            } {
                Some(value) => match value {
                    Err(e) => return Err(e),
                    Ok(_) => continue,
                },
                None => {}
            }
        }

        //? End bracket section
        if c == ')' && !in_string {
            bracket_depth -= 1;

            match bracket_depth {
                0 => {
                    symbol_line.push(match get_symbols_from_line(buffer.as_str()) {
                        Ok(symbols) => ArithmeticBlock(symbols),
                        Err(e) => return Err(e),
                    });
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

        //? End string
        if STRING_DELIMITERS.contains(&c) {
            in_string = !in_string;
        }

        //? Start bracket
        if c == '(' && !in_string {
            if bracket_depth != 0 {
                buffer.push(c);
            }
            bracket_depth += 1;
            continue;
        }

        //? End indexer
        if c == ']' && !in_string {
            if !buffer.is_empty() {
                process_buffer(&mut buffer, &mut symbol_line)?;
            }
            if !in_indexer {
                return Err(
                    "Closing indexer bracket found with no corresponding opening bracket"
                        .to_string(),
                );
            }
            if symbol_line.len() - indexing_start > 1 {
                return Err("Indexers may only contain one symbol".to_string());
            }
            if symbol_line.len() - indexing_start < 1 {
                return Err("Indexer must contain a symbol".to_string());
            }
            let symbol = symbol_line.pop().expect("Tried to pop from symbol line when empty");
            symbol_line.push(Symbol::Indexer(Box::new(symbol)));
            in_indexer = false;
            continue;
        }

        //? Start indexer
        if c == '[' && !in_string {
            if !buffer.is_empty() {
                process_buffer(&mut buffer, &mut symbol_line)?;
            }
            if in_indexer {
                return Err("Recursive indexing not permitted".to_string());
            }
            indexing_start = symbol_line.len();
            in_indexer = true;
            continue;
        }

        buffer.push(c);
    }

    if in_string {
        return Err("Unclosed string".to_string());
    }

    if bracket_depth != 0 {
        return Err("Unclosed brackets".to_string());
    }

    //? Push remaining data
    if !buffer.is_empty() {
        if let Some(symbol) = get_all_symbol(&buffer) {
            symbol_line.push(symbol);
        }
        else {
            return Err(format!("Symbol '{}' not found", buffer));
        }
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

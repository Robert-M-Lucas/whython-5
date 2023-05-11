use crate::processing::processor::MemoryManagers;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Punctuation, Symbol};
use crate::processing::types::{get_type, get_type_from_literal, Type};

//noinspection RsLift
/// Takes an evaluable expression
///
/// # Arguments
///
/// * `section` - Expression to evaluate
/// * `to_overwrite` - Type to write result to
/// * `must_evaluate` - Whether the expression must evaluate to a type
///
/// # Returns
/// * Returns `Ok(Some(Type))` if to_overwrite is `None`.
/// * Returns `Ok(None)` if to_overwrite is `Some`,
pub fn handle_arithmetic_section(
    memory_managers: &mut MemoryManagers,
    reference_stack: &ReferenceStack,
    section: &[Symbol],
    to_overwrite: Option<&Type>,
    must_evaluate: bool,
) -> Result<Option<Type>, String> {
    fn get_formatting_error() -> String {
        "Operations must be formatted:\n\
            \t[LHS] [Operator] [RHS] or\n\
            \t[Operator] [LHS] or\n\
            \t[Value] or\n\
            \t[Name][Index] or\n\
            \t[Function Name][Arguments]"
            .to_string()
    }

    if section.len() > 3 || section.is_empty() {
        return Err(get_formatting_error());
    }

    //? Three operators - [LHS] [Operator] [RHS]
    if section.len() == 3 {
        // Get operator
        let operator = match section[1] {
            Symbol::Operator(op) => op,
            _ => return Err(get_formatting_error()),
        };

        // Get lhs
        let mut _lhs_holder = None;
        let lhs = match &section[0] {
            Symbol::Name(name) => reference_stack.get_variable(name)?,
            Symbol::Literal(literal) => {
                let object = get_type_from_literal(literal, memory_managers)?;
                object.static_assign_literal(memory_managers, literal)?;
                _lhs_holder = Some(object);
                _lhs_holder.as_ref().unwrap()
            }
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(
                    memory_managers,
                    reference_stack,
                    symbols,
                    None,
                    true,
                ) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        _lhs_holder = Some(object.unwrap());
                        _lhs_holder.as_ref().unwrap()
                    }
                }
            }
            _ => {
                return Err(
                    "LHS must be a Name, Literal or an operation within brackets".to_string(),
                )
            }
        };

        // Get rhs
        let mut _rhs_holder = None;
        let rhs = match &section[2] {
            Symbol::Name(name) => reference_stack.get_variable(name)?,
            Symbol::Literal(literal) => {
                let object = match get_type_from_literal(literal, memory_managers) {
                    Err(e) => return Err(e),
                    Ok(o) => o,
                };
                object.static_assign_literal(memory_managers, literal)?;
                _rhs_holder = Some(object);
                _rhs_holder.as_ref().unwrap()
            }
            Symbol::ArithmeticBlock(symbols) => {
                match handle_arithmetic_section(
                    memory_managers,
                    reference_stack,
                    symbols,
                    None,
                    true,
                ) {
                    Err(e) => return Err(e),
                    Ok(object) => {
                        _rhs_holder = Some(object.unwrap());
                        _rhs_holder.as_ref().unwrap()
                    }
                }
            }
            _ => {
                return Err(
                    "RHS must be a Name, Literal or an operation within brackets".to_string(),
                )
            }
        };

        // Return result
        if to_overwrite.is_none() {
            let result_type = lhs.get_operation_return_type(&operator, Some(rhs))?;

            let result = get_type(&result_type, memory_managers)?;
            lhs.operate(memory_managers, operator, Some(rhs), &result)?;

            Ok(Some(result))
        } else {
            lhs.operate(memory_managers, operator, Some(rhs), to_overwrite.unwrap())?;

            Ok(None)
        }
    }
    //? Two symbols - function calling, indexing or prefix operators
    else if section.len() == 2 {
        //? Function call
        if let (Symbol::Name(name), Symbol::ArithmeticBlock(arguments)) = (&section[0], &section[1])
        {
            // Get function
            // let name = match &section[0] {
            //     Symbol::Name(name) => name.clone(),
            //     _ => panic!()
            // };

            let function = reference_stack.get_variable(name)?;

            // Get arguments
            // let arguments = match &section[1] {
            //     Symbol::ArithmeticBlock(symbols) => symbols,
            //     _ => panic!()
            // };

            let mut i: usize = 0;

            let mut argument_list = Vec::new();

            while i < arguments.len() {
                argument_list.push(
                    handle_arithmetic_section(
                        memory_managers,
                        reference_stack,
                        &[arguments[i].clone()],
                        None,
                        true,
                    )?
                    .unwrap(),
                );

                i += 1;

                if i < arguments.len() {
                    #[allow(unreachable_patterns)]
                    match arguments[i] {
                        Symbol::Punctuation(Punctuation::ListSeparator) => {}
                        _ => {
                            return Err(
                                "Arguments must be formatted ([ARGUMENT] , [ARGUMENT] , [...]"
                                    .to_string(),
                            )
                        }
                    }
                }
                i += 1
            }

            return match to_overwrite {
                Some(to_overwrite) => {
                    match function.call(
                        memory_managers,
                        argument_list.iter().collect(),
                        Some(to_overwrite),
                    ) {
                        Err(e) => Err(e),
                        Ok(_) => Ok(None),
                    }
                }
                None => {
                    return if must_evaluate {
                        // Call function with created destination
                        let return_type = match function.get_return_type() {
                            Err(e) => return Err(e),
                            Ok(value) => get_type(&value, memory_managers)?,
                        };

                        match function.call(
                            memory_managers,
                            argument_list.iter().collect(),
                            Some(&return_type),
                        ) {
                            Err(e) => Err(e),
                            Ok(_) => Ok(Some(return_type)),
                        }
                    } else {
                        // Call function without handling return
                        match function.call(memory_managers, argument_list.iter().collect(), None) {
                            Err(e) => Err(e),
                            Ok(_) => Ok(None),
                        }
                    };
                }
            };
        }
        //? Indexing
        else if matches!(section[1], Symbol::Indexer(_)) {
            // Get variable
            let to_index = match &section[0] {
                Symbol::Name(name) => reference_stack.get_variable(name)?,
                _ => return Err("Only a Name can be indexed".to_string()),
            };

            // Index
            #[allow(unused_assignments)]
            let mut type_holder = None;
            let index = match &section[1] {
                Symbol::Indexer(symbol) => match symbol.as_ref() {
                    Symbol::Name(name) => reference_stack.get_variable(name)?,
                    Symbol::Literal(literal) => {
                        match get_type_from_literal(literal, memory_managers) {
                            Err(e) => return Err(e),
                            Ok(value) => {
                                value.static_assign_literal(memory_managers, literal)?;
                                type_holder = Some(value);
                                type_holder.as_ref().unwrap()
                            }
                        }
                    }
                    _ => return Err("Name can only be indexed by a Name or a Literal".to_string()),
                },
                _ => panic!(),
            };

            if let Some(o) = to_overwrite {
                return match to_index.get_indexed(memory_managers, index, o) {
                    Err(e) => return Err(e),
                    Ok(_) => Ok(None),
                };
            } else {
                let return_type = get_type(&to_index.get_type(), memory_managers)?;

                return match to_index.get_indexed(memory_managers, index, &return_type) {
                    Err(e) => return Err(e),
                    Ok(_) => Ok(Some(return_type)),
                };
            }
        }
        //? Prefix operator e.g. '!a'
        else {
            // Get operator
            let operator = match section[0] {
                Symbol::Operator(op) => op,
                _ => return Err(get_formatting_error()),
            };

            // Get operand
            let mut _lhs_holder = None;
            let lhs = match &section[1] {
                Symbol::Name(name) => reference_stack.get_variable(name)?,
                Symbol::Literal(literal) => {
                    let object = get_type_from_literal(literal, memory_managers)?;
                    object.static_assign_literal(memory_managers, literal)?;
                    _lhs_holder = Some(object);
                    _lhs_holder.as_ref().unwrap()
                }
                Symbol::ArithmeticBlock(symbols) => {
                    match handle_arithmetic_section(
                        memory_managers,
                        reference_stack,
                        symbols,
                        None,
                        true,
                    ) {
                        Err(e) => return Err(e),
                        Ok(object) => {
                            _lhs_holder = Some(object.unwrap());
                            _lhs_holder.as_ref().unwrap()
                        }
                    }
                }
                _ => {
                    return Err(
                        "Operand must be a Name, Literal or an operation within brackets"
                            .to_string(),
                    )
                }
            };

            // Return
            if to_overwrite.is_none() {
                let result_type = lhs.get_operation_return_type(&operator, None)?;

                let result = get_type(&result_type, memory_managers)?;
                lhs.operate(memory_managers, operator, None, &result)?;

                return Ok(Some(result));
            } else {
                lhs.operate(memory_managers, operator, None, to_overwrite.unwrap())?;

                return Ok(None);
            }
        }
    }
    //? One symbol
    else {
        return match &section[0] {
            // Get type out of name
            Symbol::Name(name) => match reference_stack.get_variable(name) {
                Err(e) => return Err(e),
                Ok(value) => {
                    if to_overwrite.is_none() {
                        let object = get_type(&value.get_type(), memory_managers)?;
                        object.assign_clone(memory_managers, value)?;
                        Ok(Some(object))
                    } else {
                        to_overwrite.unwrap().assign_clone(memory_managers, value)?;
                        Ok(None)
                    }
                }
            },
            // Get type out of literal
            Symbol::Literal(literal) => {
                if to_overwrite.is_none() {
                    let object = get_type_from_literal(literal, memory_managers)?;
                    object.static_assign_literal(memory_managers, literal)?;
                    Ok(Some(object))
                } else {
                    to_overwrite
                        .unwrap()
                        .static_assign_literal(memory_managers, literal)?;
                    Ok(None)
                }
            }
            // Recurse into arithmetic block
            Symbol::ArithmeticBlock(symbols) => handle_arithmetic_section(
                memory_managers,
                reference_stack,
                symbols,
                to_overwrite,
                true,
            ),
            _ => return Err("Only a name or literal can stand alone".to_string()),
        };
    }
}

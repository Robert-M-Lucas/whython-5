use either::{Either, Left, Right};

use crate::memory::MemoryManager;
use crate::processing::blocks::StackSizes;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Operator, Symbol, TypeSymbol};
use crate::processing::types::{Type, TypeFactory};

/*
macro_rules! get_variable {
    ($output: expr, $symbol: expr, $program_memory: expr, $reference_stack: expr, $stack_sizes: expr) => {
        let store;
        $output = match $symbol {
            Symbol::Name(name) =>
                Ok($reference_stack.get_reference(name.as_str())?.get_variable()?),
            Symbol::Literal(literal) =>
                {
                    store = TypeFactory::get_default_instantiated_type_for_literal(literal, $stack_sizes, $program_memory)?;
                    Ok(&store)
                },
            Symbol::ArithmeticBlock(section) => {
                store = evaluate_arithmetic_to_any_type(section, $program_memory, $reference_stack, $stack_sizes)?;
                Ok(&store)
            }
            _ => Err("Operator must be followed by a Literal or Name".to_string())
        };
    };
}
*/

/// Takes an output name and an `Either`. The `Either` must be formatted as
/// `Either<T, &T>`. Sets output to &T.
#[macro_export]
macro_rules! unpack_either_type {
    ($output: ident, $either: expr) => {
        let temp;
        let $output = match $either {
            either::Either::Left(t) => {
                temp = t;
                &temp
            }
            either::Either::Right(t) => t,
        };
    };
}

pub enum ReturnOptions<'a> {
    /// Places the calculated value into the type - returns `None`
    ReturnIntoType(&'a Box<dyn Type>),
    /// Returns a type from the specified list
    ReturnTypes(&'a [TypeSymbol]),
    /// Returns any type
    ReturnAnyType,
}

/// Evaluates an arithmetic section and puts the result into a type
pub fn evaluate_arithmetic_into_type<'a>(
    section: &[Symbol],
    destination: &Box<dyn Type>,
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<(), String> {
    evaluate_arithmetic_section(
        section,
        &ReturnOptions::ReturnIntoType(destination),
        program_memory,
        reference_stack,
        stack_sizes,
    )?;
    Ok(())
}

/// Evaluates an arithmetic section and returns a type from the specified list
pub fn evaluate_arithmetic_to_types<'a>(
    section: &[Symbol],
    return_type_options: &[TypeSymbol],
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Either<Box<dyn Type>, &'a Box<dyn Type>>, String> {
    Ok(evaluate_arithmetic_section(
        section,
        &ReturnOptions::ReturnTypes(return_type_options),
        program_memory,
        reference_stack,
        stack_sizes,
    )?
    .unwrap())
}

/// Evaluates an arithmetic section and returns any type
pub fn evaluate_arithmetic_to_any_type<'a>(
    section: &[Symbol],
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Either<Box<dyn Type>, &'a Box<dyn Type>>, String> {
    Ok(evaluate_arithmetic_section(
        section,
        &ReturnOptions::ReturnAnyType,
        program_memory,
        reference_stack,
        stack_sizes,
    )?
    .unwrap())
}

fn evaluate_arithmetic_section<'a>(
    section: &[Symbol],
    return_options: &ReturnOptions<'_>,
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Option<Either<Box<dyn Type>, &'a Box<dyn Type>>>, String> {
    if section.is_empty() {
        return Err("Cannot evaluate a section with no symbols".to_string());
    }

    // ? No operation

    if section.len() == 1 {
        return handle_single_symbol(
            &section[0],
            return_options,
            program_memory,
            reference_stack,
            stack_sizes,
        );
    }

    match &section[0] {
        // ? Prefix operator e.g. ! A
        Symbol::Operator(operator) => {
            if section.len() != 2 {
                return Err("Operator must be followed by a Literal or Name".to_string());
            }

            let operand = handle_single_symbol(
                &section[1],
                &ReturnOptions::ReturnAnyType,
                program_memory,
                reference_stack,
                stack_sizes,
            )?
            .unwrap();

            handle_prefix_operation(
                operator,
                operand,
                return_options,
                program_memory,
                stack_sizes,
            )
        }
        // ? Normal operation e.g. A + B
        _ => {
            if section.len() != 3 {
                return Err("Arithmetic sections must be formated [Operator] [Value] or [Value] [Operator] [Value]".to_string());
            }

            let lhs = handle_single_symbol(
                &section[0],
                &ReturnOptions::ReturnAnyType,
                program_memory,
                reference_stack,
                stack_sizes,
            )?
            .unwrap();

            let operator = match &section[1] {
                Symbol::Operator(operator) => operator,
                _ => return Err("Arithmetic sections must be formated [Operator] [Value] or [Value] [Operator] [Value]".to_string())
            };

            let rhs = handle_single_symbol(
                &section[2],
                &ReturnOptions::ReturnAnyType,
                program_memory,
                reference_stack,
                stack_sizes,
            )?
            .unwrap();

            handle_operation(
                operator,
                lhs,
                rhs,
                return_options,
                program_memory,
                stack_sizes,
            )
        }
    }
}

fn incorrect_type_error(expected: &[TypeSymbol], received: &[TypeSymbol]) -> String {
    let mut expected_text = "[any]".to_string();
    if expected.len() != 0 {
        expected_text = "[".to_string();
        for e in expected {
            expected_text += (e.to_string() + ", ").as_str();
        }
        expected_text = expected_text[..expected_text.len() - 2].to_string();
    }

    let mut received_text = "[any]".to_string();
    if received.len() != 0 {
        received_text = "[".to_string();
        for r in received {
            received_text += (r.to_string() + ", ").as_str();
        }
        received_text = received_text[..received_text.len() - 2].to_string();
    }

    format!(
        "Expected type {}, received {}",
        expected_text, received_text
    )
}

fn handle_single_symbol<'a>(
    symbol: &Symbol,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    reference_stack: &'a ReferenceStack,
    stack_sizes: &mut StackSizes,
) -> Result<Option<Either<Box<dyn Type>, &'a Box<dyn Type>>>, String> {
    match symbol {
        Symbol::Name(name) => {
            let variable = reference_stack
                .get_reference(name.as_str())?
                .get_variable()?;
            match return_options {
                ReturnOptions::ReturnIntoType(output) => {
                    output.runtime_copy_from(variable)?;
                    Ok(None)
                }
                ReturnOptions::ReturnAnyType => Ok(Some(Right(variable))),
                ReturnOptions::ReturnTypes(types) => {
                    let variable_type = variable.get_type_symbol();
                    if types.len() != 0
                        && types.iter().find(|t| matches!(t, _variable_type)).is_none()
                    {
                        Err(incorrect_type_error(types, &[variable_type]))
                    } else {
                        Ok(Some(Right(variable)))
                    }
                }
            }
        }
        Symbol::Literal(literal) => {
            match return_options {
                ReturnOptions::ReturnIntoType(output) => {
                    output.runtime_copy_from_literal(literal, program_memory)?;
                    Ok(None)
                }
                ReturnOptions::ReturnAnyType => Ok(Some(Left(
                    TypeFactory::get_default_instantiated_type_for_literal(
                        literal,
                        stack_sizes,
                        program_memory,
                    )?,
                ))),
                ReturnOptions::ReturnTypes(types) => {
                    // TODO: Potentially request types from literals i.e. not default
                    let default_type = TypeFactory::get_default_instantiated_type_for_literal(
                        literal,
                        stack_sizes,
                        program_memory,
                    )?;
                    let default_type_type = default_type.get_type_symbol();
                    if types.len() != 0
                        && types
                            .iter()
                            .find(|t| matches!(t, _default_type_type))
                            .is_none()
                    {
                        Err(incorrect_type_error(types, &[default_type_type]))
                    } else {
                        Ok(Some(Left(default_type)))
                    }
                }
            }
        }
        Symbol::ArithmeticBlock(section) => evaluate_arithmetic_section(
            section,
            return_options,
            program_memory,
            reference_stack,
            stack_sizes,
        ),
        _ => Err("Expected an expression".to_string()),
    }
}

fn operator_not_implemented_error(
    lhs: &TypeSymbol,
    operator: &Operator,
    rhs: Option<&TypeSymbol>,
) -> String {
    if let Some(rhs) = rhs {
        format!("{} not supported between {} and {}", operator, lhs, rhs)
    } else {
        format!("{} not supported on {}", operator, lhs)
    }
}

// TODO: Consider removing unused arguments
fn handle_prefix_operation<'a>(
    operator: &Operator,
    operand: Either<Box<dyn Type>, &'a Box<dyn Type>>,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    stack_sizes: &mut StackSizes,
) -> Result<Option<Either<Box<dyn Type>, &'a Box<dyn Type>>>, String> {
    unpack_either_type!(operand, operand);

    match return_options {
        ReturnOptions::ReturnIntoType(output) => {
            operand.operate_prefix(operator, output, program_memory, stack_sizes)?;
            Ok(None)
        }
        ReturnOptions::ReturnAnyType => {
            let return_types = operand.get_prefix_operation_result_type(operator);
            if return_types.is_empty() {
                Err(operator_not_implemented_error(
                    &operand.get_type_symbol(),
                    operator,
                    None,
                ))
            } else {
                let mut new_type = TypeFactory::get_unallocated_type(&return_types[0])?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                operand.operate_prefix(operator, &new_type, program_memory, stack_sizes)?;

                Ok(Some(Left(new_type)))
            }
        }
        ReturnOptions::ReturnTypes(types) => {
            let return_types = operand.get_prefix_operation_result_type(operator);

            if return_types.is_empty() {
                return Err(operator_not_implemented_error(
                    &operand.get_type_symbol(),
                    operator,
                    None,
                ));
            }

            let return_type = return_types.iter().find(|_t| {
                for rt in types.iter() {
                    if matches!(rt, _t) {
                        return true;
                    }
                }
                false
            });

            if let Some(return_type) = return_type {
                let mut new_type = TypeFactory::get_unallocated_type(return_type)?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                operand.operate_prefix(operator, &new_type, program_memory, stack_sizes)?;

                Ok(Some(Left(new_type)))
            } else {
                Err(incorrect_type_error(types, &return_types))
            }
        }
    }
}

fn handle_operation<'a>(
    operator: &Operator,
    lhs: Either<Box<dyn Type>, &'a Box<dyn Type>>,
    rhs: Either<Box<dyn Type>, &'a Box<dyn Type>>,
    return_options: &ReturnOptions,
    program_memory: &mut MemoryManager,
    stack_sizes: &mut StackSizes,
) -> Result<Option<Either<Box<dyn Type>, &'a Box<dyn Type>>>, String> {
    unpack_either_type!(lhs, lhs);
    unpack_either_type!(rhs, rhs);

    match return_options {
        ReturnOptions::ReturnIntoType(output) => {
            lhs.operate(operator, rhs, output, program_memory, stack_sizes)?;
            Ok(None)
        }
        ReturnOptions::ReturnAnyType => {
            let return_types = lhs.get_operation_result_type(operator, &rhs.get_type_symbol());
            if return_types.is_empty() {
                Err(operator_not_implemented_error(
                    &lhs.get_type_symbol(),
                    operator,
                    Some(&rhs.get_type_symbol()),
                ))
            } else {
                let mut new_type = TypeFactory::get_unallocated_type(&return_types[0])?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                lhs.operate(operator, rhs, &new_type, program_memory, stack_sizes)?;

                Ok(Some(Left(new_type)))
            }
        }
        ReturnOptions::ReturnTypes(types) => {
            let return_types = lhs.get_operation_result_type(operator, &rhs.get_type_symbol());

            if return_types.is_empty() {
                return Err(operator_not_implemented_error(
                    &lhs.get_type_symbol(),
                    operator,
                    Some(&rhs.get_type_symbol()),
                ));
            }

            let return_type = return_types.iter().find(|_t| {
                for rt in types.iter() {
                    if matches!(rt, _t) {
                        return true;
                    }
                }
                false
            });

            if let Some(return_type) = return_type {
                let mut new_type = TypeFactory::get_unallocated_type(return_type)?;
                new_type.allocate_variable(stack_sizes, program_memory)?;
                lhs.operate(operator, rhs, &new_type, program_memory, stack_sizes)?;

                Ok(Some(Left(new_type)))
            } else {
                Err(incorrect_type_error(types, &return_types))
            }
        }
    }
}

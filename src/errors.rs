use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::Type;

/// Takes zero-indexed line
pub fn create_line_error<T>(error: String, line: usize) -> Result<T, String> {
    Err(format!("Line {}: {}", line + 1, error))
}

/// Creates an error explaining that the operator isn't implemented for the given type
///
/// # Error
/// `'operator' operator not implemented for 'lhs' and 'rhs'`
///
/// or
///
/// `'operator' operator not implemented for 'lhs'`
pub fn create_op_not_impl_error<T>(
    operator: &Operator,
    lhs: TypeSymbol,
    rhs: Option<&dyn Type>,
) -> Result<T, String> {
    match rhs {
        Some(rhs) => Err(format!(
            "'{}' operator not implemented for '{}' and '{}'",
            operator,
            lhs,
            rhs.get_type_symbol()
        )),
        None => Err(format!(
            "'{}' operator not implemented for '{}'",
            operator, lhs
        )),
    }
}

/// Creates an error explaining that the literal isn't implemented for the assignment of that type
///
/// # Error
/// `'literal' cannot be used to create 'type'`
pub fn create_literal_not_impl_error<T>(
    literal: &Literal,
    type_symbol: TypeSymbol,
) -> Result<T, String> {
    Err(format!("{} cannot be used to initialise {}", literal, type_symbol))
}
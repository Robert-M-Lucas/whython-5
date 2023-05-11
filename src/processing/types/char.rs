use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::not_equal_instruction_14::NotEqualInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::{get_type, Type, TypeTrait};

pub struct CharType {}

impl CharType {
    pub(crate) fn create_empty() -> Self {
        Self {}
    }
}

impl TypeTrait for CharType {
    fn static_assign_literal(
        &self,
        _super: &Type,
        memory_managers: &mut MemoryManagers,
        literal: &Literal,
    ) -> Result<(), String> {
        // Get literal value
        let value: u8;
        match literal {
            Literal::String(string) => {
                if string.len() != 1 {
                    return Err(
                        "Chars can only be assigned from StringLiterals of length 1".to_string()
                    );
                }

                value = string.chars().next().unwrap() as u8;
            }
            Literal::Int(integer) => {
                if *integer < 0 || *integer > 255 {
                    return Err("Char can be assigned from IntLiterals 0-255 only".to_string());
                }

                value = *integer as u8;
            }
            unhandled_literal => {
                return Err(format!(
                    "{} not supported for {} assignment",
                    unhandled_literal,
                    self.get_type()
                ))
            }
        }

        // Assign from constant
        let constant_address = memory_managers.variable_memory.append_byte(value);

        CopyInstruction::new_alloc(
            memory_managers,
            constant_address,
            _super.get_address(),
            self.get_size(),
        );

        Ok(())
    }

    fn create_indexed(
        &self,
        _super: &Type,
        memory_managers: &mut MemoryManagers,
        argument_literal: &Literal,
        assignment_literal: &Literal,
    ) -> Result<usize, String> {
        let count: usize = match argument_literal {
            Literal::Int(count) => match (*count).try_into() {
                Ok(value) => value,
                Err(_) => return Err(format!("Initialisation argument '{}' out of range", count)),
            },
            _ => {
                return Err(format!(
                    "This type cannot be created with {} initialisation argument",
                    argument_literal
                ))
            }
        };

        if count == 0 {
            return Err("Initialisation argument cannot be 0".to_string());
        }

        let mut assigner = match assignment_literal {
            Literal::String(string) => string.clone(),
            Literal::None => String::new(),
            _ => {
                return Err(format!(
                    "This type cannot be created with {} assignment argument",
                    assignment_literal
                ))
            }
        };

        // Fill remaining space in string with null characters
        if count > assigner.len() {
            assigner += &*"\0".repeat(count - assigner.len());
        }

        // Create every item in array
        let mut objs = Vec::with_capacity(count - 1);

        for _ in 1..count {
            let obj = get_type(&self.get_type(), memory_managers).unwrap();
            objs.push(obj);
        }

        self.static_assign_literal(
            _super,
            memory_managers,
            &Literal::String(assigner.chars().next().unwrap().to_string()),
        )?;
        for i in 1..count {
            objs[i - 1].static_assign_literal(
                memory_managers,
                &Literal::String(assigner.chars().nth(i).unwrap().to_string()),
            )?;
        }

        Ok(count)
    }

    fn get_type(&self) -> TypeSymbol {
        TypeSymbol::Character
    }

    fn get_size(&self) -> usize {
        1
    }

    fn get_operation_type(
        &self,
        _lhs: &Type,
        operator: &Operator,
        rhs: Option<&Type>,
    ) -> Result<TypeSymbol, String> {
        if rhs.is_none() {
            return create_op_not_impl_error(operator, self.get_type(), rhs);
        }

        match rhs.as_ref().unwrap().get_type() {
            TypeSymbol::Character => {}
            _ => return create_op_not_impl_error(operator, self.get_type(), rhs),
        };

        match operator {
            Operator::Equal | Operator::NotEqual => Ok(TypeSymbol::Boolean),
            _ => create_op_not_impl_error(operator, self.get_type(), rhs),
        }
    }

    fn operate(
        &self,
        lhs: &Type,
        memory_managers: &mut MemoryManagers,
        operator: Operator,
        rhs: Option<&Type>,
        destination: &Type,
    ) -> Result<(), String> {
        if rhs.is_none() {
            return create_op_not_impl_error(&operator, self.get_type(), rhs);
        }

        match rhs.as_ref().unwrap().get_type() {
            TypeSymbol::Character => {}
            _ => return create_op_not_impl_error(&operator, self.get_type(), rhs),
        };

        match operator {
            Operator::Equal => {
                EqualInstruction::new_alloc(
                    memory_managers,
                    lhs.get_address(),
                    rhs.unwrap().get_address(),
                    self.get_size(),
                    destination.get_address(),
                );
                Ok(())
            }
            Operator::NotEqual => {
                NotEqualInstruction::new_alloc(
                    memory_managers,
                    lhs.get_address(),
                    rhs.unwrap().get_address(),
                    self.get_size(),
                    destination.get_address(),
                );
                Ok(())
            }
            _ => create_op_not_impl_error(&operator, self.get_type(), rhs),
        }
    }

    fn clone(&self) -> Box<dyn TypeTrait> {
        Box::new(Self::create_empty())
    }
}

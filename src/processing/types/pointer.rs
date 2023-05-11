use crate::errors::create_op_not_impl_error;
use crate::processing::instructions::add_instruction_13::AddInstruction;
use crate::processing::instructions::copy_instruction_0::CopyInstruction;
use crate::processing::instructions::equal_instruction_7::EqualInstruction;
use crate::processing::instructions::not_equal_instruction_14::NotEqualInstruction;
use crate::processing::processor::MemoryManagers;
use crate::processing::symbols::{Literal, Operator, TypeSymbol};
use crate::processing::types::{get_type, Type, TypeTrait};
use std::mem::size_of;

pub struct PointerType {}

impl PointerType {
    pub(crate) fn create_empty() -> Self {
        Self {}
    }
}

impl TypeTrait for PointerType {
    fn static_assign_literal(
        &self,
        _super: &Type,
        memory_managers: &mut MemoryManagers,
        literal: &Literal,
    ) -> Result<(), String> {
        let value: usize;

        match literal {
            Literal::Int(integer) => {
                value = match (*integer).try_into() {
                    Err(_) => {
                        return Err(format!(
                            "Cannot fit {}'s value '{}' into Pointer",
                            literal, integer
                        ))
                    }
                    Ok(value) => value,
                }
            }
            unhandled_literal => {
                return Err(format!(
                    "{} not supported for {} assignment",
                    unhandled_literal,
                    self.get_type()
                ))
            }
        }

        let constant_address = memory_managers.variable_memory.append(&value.to_le_bytes());

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

        let assigner = match assignment_literal {
            Literal::Int(int) => *int,
            Literal::None => 0,
            _ => {
                return Err(format!(
                    "This type cannot be created with {} assignment argument",
                    assignment_literal
                ))
            }
        };

        // Assign to all objects in array
        let mut objs = Vec::with_capacity(count - 1);

        for _ in 1..count {
            let obj = get_type(&self.get_type(), memory_managers).unwrap();
            objs.push(obj);
        }

        self.static_assign_literal(_super, memory_managers, &Literal::Int(assigner))?;

        for i in 1..count {
            objs[i - 1].static_assign_literal(memory_managers, &Literal::Int(assigner))?;
        }

        Ok(count)
    }

    fn get_type(&self) -> TypeSymbol {
        TypeSymbol::Pointer
    }

    fn get_size(&self) -> usize {
        size_of::<usize>()
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
            TypeSymbol::Pointer => {}
            _ => return create_op_not_impl_error(operator, self.get_type(), rhs),
        };

        match operator {
            Operator::Equal | Operator::NotEqual => Ok(TypeSymbol::Boolean),
            Operator::Add => Ok(TypeSymbol::Pointer),
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

        let rhs = rhs.unwrap();

        match rhs.get_type() {
            TypeSymbol::Pointer => {}
            _ => return create_op_not_impl_error(&operator, self.get_type(), Some(rhs)),
        };

        match operator {
            Operator::Equal => {
                EqualInstruction::new_alloc(
                    memory_managers,
                    lhs.get_address(),
                    rhs.get_address(),
                    self.get_size(),
                    destination.get_address(),
                );
                Ok(())
            }
            Operator::NotEqual => {
                NotEqualInstruction::new_alloc(
                    memory_managers,
                    lhs.get_address(),
                    rhs.get_address(),
                    self.get_size(),
                    destination.get_address(),
                );
                Ok(())
            }
            Operator::Add => {
                AddInstruction::new_alloc(
                    memory_managers,
                    lhs.get_address(),
                    rhs.get_address(),
                    self.get_size(),
                    destination.get_address(),
                );
                Ok(())
            }
            _ => create_op_not_impl_error(&operator, self.get_type(), Some(rhs)),
        }
    }

    fn clone(&self) -> Box<dyn TypeTrait> {
        Box::new(Self::create_empty())
    }
}

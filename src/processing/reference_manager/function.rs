use crate::address::Address;
use crate::memory::MemoryManager;
use crate::processing::arithmetic::evaluate_arithmetic_to_types;
use crate::processing::blocks::StackSizes;
use crate::processing::instructions::copy_3::CopyInstruction;
use crate::processing::instructions::jump_instruction_10::JumpInstruction;
use crate::processing::instructions::stack_create_0::StackCreateInstruction;
use crate::processing::instructions::stack_down_4::StackDownInstruction;
use crate::processing::reference_manager::ReferenceStack;
use crate::processing::symbols::{Literal, Symbol};
use crate::processing::types::pointer::PointerType;
use crate::processing::types::Type;
use crate::util::must_use_option::MustUseOption;
use crate::util::warn;

#[must_use]
pub struct IncompleteFunctionCall {
    stack_create_instruction: StackCreateInstruction,
    copy_instructions_to_offset: Vec<CopyInstruction>,
}

impl IncompleteFunctionCall {
    pub fn new(
        stack_create_instruction: StackCreateInstruction,
        copy_instructions_to_offset: Vec<CopyInstruction>,
    ) -> Self {
        Self {
            stack_create_instruction,
            copy_instructions_to_offset,
        }
    }
}

pub struct FunctionReference {
    jump_address: usize,
    return_pointer: PointerType,
    parameters: Vec<(String, Box<dyn Type>)>,
    stack_size: Option<usize>,
    incomplete_function_calls: Vec<IncompleteFunctionCall>,
}

impl FunctionReference {
    pub fn new(
        jump_address: usize,
        return_pointer: PointerType,
        parameters: Vec<(String, Box<dyn Type>)>,
        stack_size: Option<usize>,
    ) -> Self {
        Self {
            jump_address,
            return_pointer,
            parameters,
            stack_size,
            incomplete_function_calls: Vec::new(),
        }
    }

    /// Finishes the construction of all `IncompleteFunctionCall`s that required the stack size of
    /// the function to work properly
    pub fn set_stack_size_and_complete(
        &mut self,
        stack_size: usize,
        program_memory: &mut MemoryManager,
    ) {
        self.stack_size = Some(stack_size);
        self.complete(program_memory);
    }

    fn complete(&mut self, program_memory: &mut MemoryManager) {
        for to_complete in &mut self.incomplete_function_calls {
            to_complete
                .stack_create_instruction
                .set_stack_size(self.stack_size.unwrap(), program_memory);
            for copy_instruction in &to_complete.copy_instructions_to_offset {
                let mut address = Address::stack_address_from_bytes(
                    copy_instruction.get_source_address(),
                    &program_memory.memory,
                )
                .unwrap();
                address.offset_if_stack(self.stack_size.unwrap());
                copy_instruction.set_source(&address, program_memory);
            }
        }

        self.incomplete_function_calls = Vec::new();
    }

    /// Calls the function. If the call is recursive (i.e. the stack size of the function is not
    /// known yet), this returns an `IncompleteFunctionCall` that must be handled with the
    /// `add_incomplete_function_call` method
    pub fn call(
        &self,
        _return_into: Option<&dyn Type>,
        arguments: &Vec<Vec<Symbol>>,
        program_memory: &mut MemoryManager,
        reference_stack: &ReferenceStack,
        stack_sizes: &mut StackSizes,
    ) -> Result<MustUseOption<IncompleteFunctionCall>, String> {
        // Check number of arguments
        if arguments.len() != self.parameters.len() {
            return Err(format!(
                "Expected {} arguments - received {}",
                self.parameters.len(),
                arguments.len()
            ));
        }

        // Evaluate arguments to intermediate type
        let mut intermediate = Vec::with_capacity(self.parameters.len());
        for (argument, parameter) in arguments.iter().zip(&self.parameters) {
            intermediate.push(evaluate_arithmetic_to_types(
                argument,
                &[parameter.1.get_type_symbol()],
                program_memory,
                reference_stack,
                stack_sizes,
            )?);
        }

        // Create stack
        let stack_create_instruction;
        if let Some(stack_size) = self.stack_size {
            stack_create_instruction =
                StackCreateInstruction::new_alloc(program_memory, stack_size);
        } else {
            stack_create_instruction = StackCreateInstruction::new_alloc(program_memory, 0);
        }

        let mut copy_instructions_to_offset = Vec::new();

        // Copy intermediate types into new stack
        for (i, t) in intermediate.into_iter().enumerate() {
            let t = t.as_ref();
            let mut t = t.duplicate(); // Allow mutability
            if let Some(stack_size) = self.stack_size {
                t.get_address_mut().offset_if_stack(stack_size); // Offset to account for new stack
                self.parameters[i]
                    .1
                    .runtime_copy_from(t.as_ref(), program_memory)?; // Copy into parameter
            } else {
                // Copy into parameter
                copy_instructions_to_offset.push(
                    self.parameters[i]
                        .1
                        .runtime_copy_from(t.as_ref(), program_memory)?,
                );
            }
        }

        // Copy return address
        let copy_instruction = self
            .return_pointer
            .runtime_copy_from_literal(&Literal::Int(0), program_memory)?;
        // self.return_pointer.runtime_copy_from_literal(&Literal::Int((165) as i128), program_memory)?;

        // Jump to function
        JumpInstruction::new_alloc(program_memory, self.jump_address);

        copy_instruction.set_source(
            &Address::Immediate(Vec::from(program_memory.get_position().to_le_bytes())),
            program_memory,
        );

        StackDownInstruction::new_alloc(program_memory);

        if self.stack_size.is_none() {
            Ok(MustUseOption::Some(IncompleteFunctionCall::new(
                stack_create_instruction,
                copy_instructions_to_offset,
            )))
        } else {
            Ok(MustUseOption::None)
        }
    }

    /// Adds an `IncompleteFunctionCall` to an internal list to be completed when the stack size
    /// of the function is determined
    pub fn add_incomplete_function_call(
        &mut self,
        incomplete_function_call: IncompleteFunctionCall,
    ) {
        self.incomplete_function_calls
            .push(incomplete_function_call)
    }
}

#[cfg(debug_assertions)]
impl Drop for FunctionReference {
    fn drop(&mut self) {
        if self.stack_size.is_none() {
            warn("Function was dropped without the stack size being determined (possible dangling IncompleteFunctionCalls)");
        }
    }
}

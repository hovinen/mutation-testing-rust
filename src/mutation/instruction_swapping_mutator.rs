use crate::mutation::mutation::Mutation;
use crate::mutation::mutator::Mutator;
use parity_wasm::elements::{FuncBody, Instruction};

#[derive(Clone)]
pub struct InstructionSwappingMutator {
    pub(crate) original_instruction: Instruction,
    pub(crate) replacement_instruction: Instruction,
}

impl Mutator for InstructionSwappingMutator {
    fn perform(&self, body: &mut FuncBody, index: usize) {
        body.code_mut().elements_mut()[index] = self.replacement_instruction.clone();
    }

    fn find(&self, body: &FuncBody, function_index: usize) -> Vec<Mutation> {
        let mut result = Vec::<Mutation>::new();
        for (instruction_index, instruction) in body.code().elements().iter().enumerate() {
            if *instruction == self.original_instruction {
                result.push(Mutation {
                    mutator: Box::new(self.clone()),
                    instruction_index,
                    function_index,
                });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::mutation::instruction_swapping_mutator::InstructionSwappingMutator;
    use crate::mutation::Mutator;
    use parity_wasm::builder::{FuncBodyBuilder, Identity, ModuleBuilder};
    use parity_wasm::elements::{Instruction, Instructions};

    #[test]
    fn perform_writes_new_instruction_at_index_0() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();

        subject.perform(&mut body, 0);

        assert_eq!(body.code().elements(), vec![Instruction::I32GtU]);
    }

    #[test]
    fn perform_writes_new_instruction_at_index_1() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::I32Add,
                Instruction::I32GeU,
            ]))
            .build();

        subject.perform(&mut body, 1);

        assert_eq!(
            body.code().elements(),
            vec![Instruction::I32Add, Instruction::I32GtU]
        );
    }

    #[test]
    fn find_does_not_identify_mutation_in_empty_function() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![]))
            .build();

        let result = subject.find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_does_not_identify_mutation_in_function_with_no_matching_instruction() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32Add]))
            .build();

        let result = subject.find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_identifies_mutation_at_index_0() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();
        let mut module = ModuleBuilder::with_callback(Identity)
            .function()
            .with_body(body)
            .build()
            .build();

        let result = subject.find(&module.code_section().unwrap().bodies()[0], 0);

        result[0].perform(&mut module);
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::I32GtU]
        );
    }

    #[test]
    fn find_identifies_mutation_at_index_1() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::I32Add,
                Instruction::I32GeU,
            ]))
            .build();
        let mut module = ModuleBuilder::with_callback(Identity)
            .function()
            .with_body(body)
            .build()
            .build();

        let result = subject.find(&module.code_section().unwrap().bodies()[0], 0);

        result[0].perform(&mut module);
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::I32Add, Instruction::I32GtU]
        );
    }

    #[test]
    fn find_identifies_mutation_at_function_index_1() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();
        let mut module = ModuleBuilder::with_callback(Identity)
            .function()
            .build()
            .function()
            .with_body(body)
            .build()
            .build();

        let result = subject.find(&module.code_section().unwrap().bodies()[1], 1);

        result[0].perform(&mut module);
        assert_eq!(
            module.code_section().unwrap().bodies()[1].code().elements(),
            vec![Instruction::I32GtU]
        );
    }

    #[test]
    fn find_identifies_two_mutations() {
        let subject = InstructionSwappingMutator {
            original_instruction: Instruction::I32GeU,
            replacement_instruction: Instruction::I32GtU,
        };
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::I32GeU,
                Instruction::I32GeU,
            ]))
            .build();
        let mut module = ModuleBuilder::with_callback(Identity)
            .function()
            .with_body(body)
            .build()
            .build();

        let result = subject.find(&module.code_section().unwrap().bodies()[0], 0);

        for mutation in result {
            mutation.perform(&mut module);
        }
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::I32GtU, Instruction::I32GtU]
        );
    }
}

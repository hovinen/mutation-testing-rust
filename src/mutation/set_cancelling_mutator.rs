use crate::mutation::mutator::Mutator;
use parity_wasm::elements::{FuncBody, Instruction};
use crate::mutation::Mutation;

#[derive(Clone)]
pub(crate) struct SetCancellingMutator;

impl Mutator for SetCancellingMutator {
    fn perform(&self, body: &mut FuncBody, index: usize) {
        body.code_mut().elements_mut()[index] = Instruction::Drop;
    }

    fn find(&self, body: &FuncBody, function_index: usize) -> Vec<Mutation> {
        let mut result = Vec::<Mutation>::new();
        for (instruction_index, instruction) in body.code().elements().iter().enumerate() {
            if let Instruction::SetLocal(_) = *instruction {
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
    use crate::mutation::set_cancelling_mutator::SetCancellingMutator;
    use parity_wasm::builder::{FuncBodyBuilder, Identity, ModuleBuilder};
    use parity_wasm::elements::{Instructions, Instruction};
    use crate::mutation::mutator::Mutator;

    #[test]
    fn perform_writes_new_instruction_at_index_0() {
        let subject = SetCancellingMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::SetLocal(0)]))
            .build();

        subject.perform(&mut body, 0);

        assert_eq!(body.code().elements(), vec![Instruction::Drop]);
    }

    #[test]
    fn perform_writes_new_instruction_at_index_1() {
        let subject = SetCancellingMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::GetLocal(1),
                Instruction::SetLocal(0),
            ]))
            .build();

        subject.perform(&mut body, 1);

        assert_eq!(
            body.code().elements(),
            vec![Instruction::GetLocal(1), Instruction::Drop]
        );
    }

    #[test]
    fn find_does_not_identify_mutation_in_empty_function() {
        let subject = SetCancellingMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![]))
            .build();

        let result = subject.find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_does_not_identify_mutation_in_function_with_no_matching_instruction() {
        let subject = SetCancellingMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32Add]))
            .build();

        let result = subject.find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_identifies_mutation_at_index_0() {
        let subject = SetCancellingMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::SetLocal(0)]))
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
            vec![Instruction::Drop]
        );
    }
    #[test]
    fn find_identifies_mutation_at_index_1() {
        let subject = SetCancellingMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::I32Add,
                Instruction::SetLocal(1),
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
            vec![Instruction::I32Add, Instruction::Drop]
        );
    }

    #[test]
    fn find_identifies_mutation_at_function_index_1() {
        let subject = SetCancellingMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::SetLocal(2)]))
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
            vec![Instruction::Drop]
        );
    }

    #[test]
    fn find_identifies_two_mutations() {
        let subject = SetCancellingMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::SetLocal(0),
                Instruction::SetLocal(1),
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
            vec![Instruction::Drop, Instruction::Drop]
        );
    }
}

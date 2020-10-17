use crate::mutation::mutator::Mutator;
use crate::mutation::Mutation;
use parity_wasm::elements::{FuncBody, Instruction};

#[derive(Clone)]
pub(crate) struct IfConditionToFalseMutator;

impl Mutator for IfConditionToFalseMutator {
    fn perform(&self, body: &mut FuncBody, index: usize) {
        let elements = body.code_mut().elements_mut();
        elements[index] = Instruction::Drop;
    }

    fn find(&self, body: &FuncBody, function_index: usize) -> Vec<Mutation> {
        let mut result = Vec::<Mutation>::new();
        for (instruction_index, instruction) in body.code().elements().iter().enumerate() {
            match *instruction {
                Instruction::BrIf(_) => {
                    result.push(self.create_mutation(function_index, instruction_index))
                }
                _ => {}
            }
        }
        result
    }
}

impl IfConditionToFalseMutator {
    fn create_mutation(&self, function_index: usize, instruction_index: usize) -> Mutation {
        Mutation {
            mutator: Box::new(self.clone()),
            instruction_index,
            function_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mutation::mutator::Mutator;
    use crate::mutation::if_condition_to_false_mutator::IfConditionToFalseMutator;
    use parity_wasm::builder::{FuncBodyBuilder, Identity, ModuleBuilder};
    use parity_wasm::elements::{Instruction, Instructions};

    #[test]
    fn perform_writes_new_instruction_at_index_0() {
        let subject = IfConditionToFalseMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::BrIf(0)]))
            .build();

        subject.perform(&mut body, 0);

        assert_eq!(body.code().elements(), vec![Instruction::Drop]);
    }

    #[test]
    fn perform_writes_new_instruction_at_index_1() {
        let subject = IfConditionToFalseMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::GetLocal(1),
                Instruction::BrIf(0),
            ]))
            .build();

        subject.perform(&mut body, 1);

        assert_eq!(
            body.code().elements(),
            vec![Instruction::GetLocal(1), Instruction::Drop]
        );
    }

    #[test]
    fn perform_inserts_drop_instruction_where_required() {
        let subject = IfConditionToFalseMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::GetLocal(0),
                Instruction::BrIf(0),
                Instruction::GetLocal(0),
            ]))
            .build();

        subject.perform(&mut body, 1);

        assert_eq!(
            body.code().elements(),
            vec![
                Instruction::GetLocal(0),
                Instruction::Drop,
                Instruction::GetLocal(0)
            ]
        );
    }

    #[test]
    fn find_does_not_identify_mutation_in_empty_function() {
        let subject = IfConditionToFalseMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![]))
            .build();

        let result = subject.find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_does_not_identify_mutation_in_function_with_no_matching_instruction() {
        let subject = IfConditionToFalseMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32Add]))
            .build();

        let result = subject.find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_identifies_mutation_index_0() {
        let subject = IfConditionToFalseMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::BrIf(0)]))
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
        let subject = IfConditionToFalseMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::I32Add,
                Instruction::BrIf(0),
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
        let subject = IfConditionToFalseMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::BrIf(0)]))
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
        let subject = IfConditionToFalseMutator;
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::BrIf(0),
                Instruction::BrIf(1),
            ]))
            .build();
        let mut module = ModuleBuilder::with_callback(Identity)
            .function()
            .with_body(body)
            .build()
            .build();

        let result = subject.find(&module.code_section().unwrap().bodies()[0], 0);

        for mutation in result.iter().rev() {
            mutation.perform(&mut module);
        }
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::Drop, Instruction::Drop]
        );
    }
}

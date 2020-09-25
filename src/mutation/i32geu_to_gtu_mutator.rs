use crate::mutation::mutation::Mutation;
use crate::mutation::mutator::Mutator;
use parity_wasm::elements::{FuncBody, Instruction};

pub struct I32GeUToGtUMutator;

impl Mutator for I32GeUToGtUMutator {
    fn perform(&self, body: &mut FuncBody, index: usize) {
        body.code_mut().elements_mut()[index] = Instruction::I32GtU;
    }
}

impl I32GeUToGtUMutator {
    pub fn find(body: &FuncBody, function_index: usize) -> Vec<Mutation> {
        let mut result = Vec::<Mutation>::new();
        for (instruction_index, instruction) in body.code().elements().iter().enumerate() {
            if *instruction == Instruction::I32GeU {
                result.push(Mutation {
                    mutator: Box::new(I32GeUToGtUMutator),
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
    use crate::mutation::{I32GeUToGtUMutator, Mutator};
    use parity_wasm::builder::{FuncBodyBuilder, Identity, ModuleBuilder};
    use parity_wasm::elements::{Instruction, Instructions};

    #[test]
    fn perform_writes_new_instruction_at_index_0() {
        let subject = I32GeUToGtUMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();

        subject.perform(&mut body, 0);

        assert_eq!(body.code().elements(), vec![Instruction::I32GtU]);
    }

    #[test]
    fn perform_writes_new_instruction_at_index_1() {
        let subject = I32GeUToGtUMutator;
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
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![]))
            .build();

        let result = I32GeUToGtUMutator::find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_does_not_identify_mutation_in_function_with_no_matching_instruction() {
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32Add]))
            .build();

        let result = I32GeUToGtUMutator::find(&body, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn find_identifies_mutation_at_index_0() {
        let body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();
        let mut module = ModuleBuilder::with_callback(Identity)
            .function()
            .with_body(body)
            .build()
            .build();

        let result = I32GeUToGtUMutator::find(&module.code_section().unwrap().bodies()[0], 0);

        result[0].perform(&mut module);
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::I32GtU]
        );
    }

    #[test]
    fn find_identifies_mutation_at_index_1() {
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

        let result = I32GeUToGtUMutator::find(&module.code_section().unwrap().bodies()[0], 0);

        result[0].perform(&mut module);
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::I32Add, Instruction::I32GtU]
        );
    }

    #[test]
    fn find_identifies_mutation_at_function_index_1() {
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

        let result = I32GeUToGtUMutator::find(&module.code_section().unwrap().bodies()[1], 1);

        result[0].perform(&mut module);
        assert_eq!(
            module.code_section().unwrap().bodies()[1].code().elements(),
            vec![Instruction::I32GtU]
        );
    }

    #[test]
    fn find_identifies_two_mutations() {
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

        let result = I32GeUToGtUMutator::find(&module.code_section().unwrap().bodies()[0], 0);

        for mutation in result {
            mutation.perform(&mut module);
        }
        assert_eq!(
            module.code_section().unwrap().bodies()[0].code().elements(),
            vec![Instruction::I32GtU, Instruction::I32GtU]
        );
    }
}

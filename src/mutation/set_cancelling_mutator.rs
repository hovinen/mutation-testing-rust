use crate::mutation::mutator::Mutator;
use crate::mutation::Mutation;
use parity_wasm::elements::{FuncBody, Instruction};

#[derive(Clone)]
pub(crate) struct SetCancellingMutator;

impl Mutator for SetCancellingMutator {
    fn perform(&self, body: &mut FuncBody, index: usize) {
        match body.code_mut().elements_mut()[index] {
            Instruction::SetLocal(_) | Instruction::SetGlobal(_) => {
                body.code_mut().elements_mut()[index] = Instruction::Drop;
            }
            Instruction::I32Store(_, _)
            | Instruction::I64Store(_, _)
            | Instruction::F32Store(_, _)
            | Instruction::F64Store(_, _) => {
                body.code_mut().elements_mut()[index] = Instruction::Drop;
                body.code_mut()
                    .elements_mut()
                    .insert(index, Instruction::Drop);
            }
            _ => {}
        }
    }

    fn find(&self, body: &FuncBody, function_index: usize) -> Vec<Mutation> {
        let mut result = Vec::<Mutation>::new();
        for (instruction_index, instruction) in body.code().elements().iter().enumerate() {
            match *instruction {
                Instruction::SetLocal(_)
                | Instruction::SetGlobal(_)
                | Instruction::I32Store(_, _)
                | Instruction::I64Store(_, _)
                | Instruction::F32Store(_, _)
                | Instruction::F64Store(_, _) => {
                    result.push(self.create_mutation(function_index, instruction_index))
                }
                _ => {}
            }
        }
        result
    }

    fn describe(&self) -> String {
        String::from("SetCancelling")
    }
}

impl SetCancellingMutator {
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
    use crate::mutation::set_cancelling_mutator::SetCancellingMutator;
    use parity_wasm::builder::{FuncBodyBuilder, Identity, ModuleBuilder};
    use parity_wasm::elements::{Instruction, Instructions};

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
    fn perform_inserts_drop_instruction_where_required() {
        let subject = SetCancellingMutator;
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![
                Instruction::GetLocal(0),
                Instruction::I32Store(0, 0),
                Instruction::GetLocal(0),
            ]))
            .build();

        subject.perform(&mut body, 1);

        assert_eq!(
            body.code().elements(),
            vec![
                Instruction::GetLocal(0),
                Instruction::Drop,
                Instruction::Drop,
                Instruction::GetLocal(0)
            ]
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

    macro_rules! mutation_tests {
        ($($name:ident: $instruction:expr, $replacement:expr,)*) => {
        mod find_identifies_mutation_index_0 {
            use crate::mutation::set_cancelling_mutator::SetCancellingMutator;
            use parity_wasm::builder::{FuncBodyBuilder, Identity, ModuleBuilder};
            use parity_wasm::elements::{Instructions, Instruction};
            use crate::mutation::mutator::Mutator;

            $(
                #[test]
                fn $name() {
                    let subject = SetCancellingMutator;
                    let body = FuncBodyBuilder::with_callback(Identity)
                        .with_instructions(Instructions::new(vec![$instruction]))
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
                        $replacement
                    );
                }
            )*
        }
        }
    }

    mutation_tests! {
        set_local: Instruction::SetLocal(0), vec![Instruction::Drop],
        set_global: Instruction::SetGlobal(0), vec![Instruction::Drop],
        i32_store: Instruction::I32Store(0, 0), vec![Instruction::Drop, Instruction::Drop],
        i64_store: Instruction::I64Store(0, 0), vec![Instruction::Drop, Instruction::Drop],
        f32_store: Instruction::F32Store(0, 0), vec![Instruction::Drop, Instruction::Drop],
        f64_store: Instruction::F64Store(0, 0), vec![Instruction::Drop, Instruction::Drop],
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

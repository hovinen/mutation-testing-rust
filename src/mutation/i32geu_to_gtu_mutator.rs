use crate::mutation::mutation::Mutation;
use crate::mutation::instruction_swapping_mutator::InstructionSwappingMutator;
use crate::mutation::mutator::Mutator;
use parity_wasm::elements::{FuncBody, Instruction};

pub struct I32GeUToGtUMutator {
    mutator: InstructionSwappingMutator
}

impl Mutator for I32GeUToGtUMutator {
    fn perform(&self, body: &mut FuncBody, index: usize) {
        self.mutator.perform(body, index)
    }
}

impl I32GeUToGtUMutator {
    pub fn create() -> Self {
        I32GeUToGtUMutator {
            mutator: InstructionSwappingMutator {
                original_instruction: Instruction::I32GeU,
                replacement_instruction: Instruction::I32GtU
            }
        }
    }

    pub fn find(body: &FuncBody, function_index: usize) -> Vec<Mutation> {
        I32GeUToGtUMutator::create().mutator.find(body, function_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::mutation::{I32GeUToGtUMutator, Mutator};
    use parity_wasm::builder::{FuncBodyBuilder, Identity};
    use parity_wasm::elements::{Instruction, Instructions};

    #[test]
    fn perform_writes_new_instruction_at_index_0() {
        let subject = I32GeUToGtUMutator::create();
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();

        subject.perform(&mut body, 0);

        assert_eq!(body.code().elements(), vec![Instruction::I32GtU]);
    }
}

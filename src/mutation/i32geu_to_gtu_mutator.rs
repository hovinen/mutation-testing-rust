use crate::mutation::instruction_swapping_mutator::InstructionSwappingMutator;
use parity_wasm::elements::Instruction;

pub(crate) static MUTATOR: InstructionSwappingMutator = InstructionSwappingMutator {
    original_instruction: Instruction::I32GeU,
    replacement_instruction: Instruction::I32GtU,
};

#[cfg(test)]
mod tests {
    use crate::mutation::i32geu_to_gtu_mutator::MUTATOR;
    use crate::mutation::Mutator;
    use parity_wasm::builder::{FuncBodyBuilder, Identity};
    use parity_wasm::elements::{Instruction, Instructions};

    #[test]
    fn perform_writes_new_instruction_at_index_0() {
        let mut body = FuncBodyBuilder::with_callback(Identity)
            .with_instructions(Instructions::new(vec![Instruction::I32GeU]))
            .build();

        MUTATOR.perform(&mut body, 0);

        assert_eq!(body.code().elements(), vec![Instruction::I32GtU]);
    }
}

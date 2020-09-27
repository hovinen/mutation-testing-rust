use crate::mutation::Mutator;
use crate::mutation::instruction_swapping_mutator::InstructionSwappingMutator;
use parity_wasm::elements::Instruction;

pub(crate) static ALL_MUTATORS: [&'static (dyn Mutator + Send + Sync + 'static); 1] = [
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32GeU,
        replacement_instruction: Instruction::I32GtU,
    }
];

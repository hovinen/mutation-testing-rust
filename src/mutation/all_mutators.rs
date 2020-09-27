use crate::mutation::Mutator;
use crate::mutation::instruction_swapping_mutator::InstructionSwappingMutator;
use parity_wasm::elements::Instruction;

pub(crate) static ALL_MUTATORS: [&'static (dyn Mutator + Send + Sync + 'static); 12] = [
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32GeU,
        replacement_instruction: Instruction::I32GtU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32LeU,
        replacement_instruction: Instruction::I32LtU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32GtU,
        replacement_instruction: Instruction::I32GeU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32LtU,
        replacement_instruction: Instruction::I32LeU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32GeS,
        replacement_instruction: Instruction::I32GtS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32LeS,
        replacement_instruction: Instruction::I32LtS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32GtS,
        replacement_instruction: Instruction::I32GeS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32LtS,
        replacement_instruction: Instruction::I32LeS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Ge,
        replacement_instruction: Instruction::F32Gt,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Gt,
        replacement_instruction: Instruction::F32Ge
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Ge,
        replacement_instruction: Instruction::F64Gt,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Gt,
        replacement_instruction: Instruction::F64Ge,
    },
];

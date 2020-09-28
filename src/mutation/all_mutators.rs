use crate::mutation::instruction_swapping_mutator::InstructionSwappingMutator;
use crate::mutation::set_cancelling_mutator::SetCancellingMutator;
use crate::mutation::Mutator;
use parity_wasm::elements::Instruction;

pub(crate) static ALL_MUTATORS: [&'static (dyn Mutator + Send + Sync + 'static); 53] = [
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
        original_instruction: Instruction::I32Eq,
        replacement_instruction: Instruction::I32Ne,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32Ne,
        replacement_instruction: Instruction::I32Eq,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32Add,
        replacement_instruction: Instruction::I32Sub,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32Sub,
        replacement_instruction: Instruction::I32Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32Add,
        replacement_instruction: Instruction::I32Mul,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32Mul,
        replacement_instruction: Instruction::I32Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32And,
        replacement_instruction: Instruction::I32Or,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I32Or,
        replacement_instruction: Instruction::I32And,
    },

    &InstructionSwappingMutator {
        original_instruction: Instruction::I64GeU,
        replacement_instruction: Instruction::I64GtU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64LeU,
        replacement_instruction: Instruction::I64LtU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64GtU,
        replacement_instruction: Instruction::I64GeU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64LtU,
        replacement_instruction: Instruction::I64LeU,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64GeS,
        replacement_instruction: Instruction::I64GtS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64LeS,
        replacement_instruction: Instruction::I64LtS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64GtS,
        replacement_instruction: Instruction::I64GeS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64LtS,
        replacement_instruction: Instruction::I64LeS,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Eq,
        replacement_instruction: Instruction::I64Ne,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Ne,
        replacement_instruction: Instruction::I64Eq,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Add,
        replacement_instruction: Instruction::I64Sub,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Sub,
        replacement_instruction: Instruction::I64Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Add,
        replacement_instruction: Instruction::I64Mul,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Mul,
        replacement_instruction: Instruction::I64Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64And,
        replacement_instruction: Instruction::I64Or,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::I64Or,
        replacement_instruction: Instruction::I64And,
    },

    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Ge,
        replacement_instruction: Instruction::F32Gt,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Gt,
        replacement_instruction: Instruction::F32Ge,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Eq,
        replacement_instruction: Instruction::F32Ne,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Ne,
        replacement_instruction: Instruction::F32Eq,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Add,
        replacement_instruction: Instruction::F32Sub,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Sub,
        replacement_instruction: Instruction::F32Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Add,
        replacement_instruction: Instruction::F32Mul,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Mul,
        replacement_instruction: Instruction::F32Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Ceil,
        replacement_instruction: Instruction::F32Floor,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F32Floor,
        replacement_instruction: Instruction::F32Ceil,
    },

    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Ge,
        replacement_instruction: Instruction::F64Gt,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Gt,
        replacement_instruction: Instruction::F64Ge,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Eq,
        replacement_instruction: Instruction::F64Ne,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Ne,
        replacement_instruction: Instruction::F64Eq,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Add,
        replacement_instruction: Instruction::F64Sub,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Sub,
        replacement_instruction: Instruction::F64Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Add,
        replacement_instruction: Instruction::F64Mul,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Mul,
        replacement_instruction: Instruction::F64Add,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Ceil,
        replacement_instruction: Instruction::F64Floor,
    },
    &InstructionSwappingMutator {
        original_instruction: Instruction::F64Floor,
        replacement_instruction: Instruction::F64Ceil,
    },

    &SetCancellingMutator,
];

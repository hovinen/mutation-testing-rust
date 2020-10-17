use crate::mutation::mutator::Mutator;
use parity_wasm::elements::Module;
use std::fmt::{Debug, Formatter, Result};

pub struct Mutation {
    pub(crate) mutator: Box<dyn Mutator>,
    pub(crate) instruction_index: usize,
    pub(crate) function_index: usize,
}

impl Mutation {
    pub fn perform(&self, module: &mut Module) {
        self.mutator.perform(
            &mut module.code_section_mut().unwrap().bodies_mut()[self.function_index],
            self.instruction_index,
        );
    }
}

impl Debug for Mutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\nMutation<{}, {}, {}>", self.mutator.describe(), self.function_index, self.instruction_index)
    }
}

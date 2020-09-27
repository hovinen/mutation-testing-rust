use crate::mutation::Mutation;
use parity_wasm::elements::FuncBody;

pub(crate) trait Mutator {
    fn perform(&self, body: &mut FuncBody, index: usize);

    fn find(&self, body: &FuncBody, function_index: usize) -> Vec<Mutation>;
}

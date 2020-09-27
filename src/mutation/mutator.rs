use parity_wasm::elements::FuncBody;
use crate::mutation::Mutation;

pub trait Mutator {
    fn perform(&self, body: &mut FuncBody, index: usize);

    fn find(&self, body: &FuncBody, function_index: usize) -> Vec<Mutation>;
}

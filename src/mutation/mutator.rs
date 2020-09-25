use parity_wasm::elements::FuncBody;

pub trait Mutator {
    fn perform(&self, body: &mut FuncBody, index: usize);
}

pub mod i32geu_to_gtu_mutator;
pub mod instruction_swapping_mutator;
pub mod mutation;
pub mod mutator;

pub use i32geu_to_gtu_mutator::I32GeUToGtUMutator;
pub use mutation::Mutation;
pub use mutator::Mutator;
use parity_wasm::elements::FuncBody;

pub(crate) fn find_mutations(indices: Vec<usize>, bodies: &[FuncBody]) -> Vec<Mutation> {
    let mut mutations = Vec::<Mutation>::new();
    for index in indices.iter() {
        let body = &bodies[*index];
        mutations.append(&mut I32GeUToGtUMutator::find(body, *index));
    }
    mutations
}

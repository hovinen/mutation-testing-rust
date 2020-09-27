mod instruction_swapping_mutator;
pub mod mutation;
mod mutator;
mod all_mutators;

pub use mutation::Mutation;
use mutator::Mutator;
use parity_wasm::elements::FuncBody;

pub(crate) fn find_mutations(indices: Vec<usize>, bodies: &[FuncBody]) -> Vec<Mutation> {
    let mut mutations = Vec::<Mutation>::new();
    for index in indices.iter() {
        let body = &bodies[*index];
        for mutator in all_mutators::ALL_MUTATORS.iter() {
            mutations.append(&mut mutator.find(body, *index));
        }
    }
    mutations
}

use crate::mutation::Mutation;
use crate::runner::TestResult;
use crate::{candidates, mutation, runner};
use parity_wasm::deserialize_buffer;
use parity_wasm::elements::Module;

pub fn find_surviving_mutants(
    original_module_contents: &[u8],
    include_modules: &[&str],
    exclude_modules: &[&str],
) -> Vec<Mutation> {
    let module = load_module(original_module_contents);
    let indices =
        candidates::identify_candidate_functions(&module, include_modules, exclude_modules);
    let code_section = module.code_section().unwrap();
    let bodies = code_section.bodies();
    let mutations = mutation::find_mutations(indices, bodies);
    run_mutations(original_module_contents, mutations)
}

fn run_mutations(original_module_contents: &[u8], mutations: Vec<Mutation>) -> Vec<Mutation> {
    let mut surviving_mutants = Vec::new();
    for mutation in mutations {
        let mut mutated_module = load_module(original_module_contents);
        mutation.perform(&mut mutated_module);
        let serialized = mutated_module.to_bytes().unwrap();
        if runner::run_tests(serialized.as_slice()) == TestResult::Passed {
            surviving_mutants.push(mutation);
        }
    }
    surviving_mutants
}

fn load_module(contents: &[u8]) -> Module {
    deserialize_buffer::<Module>(contents)
        .unwrap()
        .parse_names()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::engine::find_surviving_mutants;

    #[test]
    fn reports_no_surviving_mutants_when_no_mutants_found() {
        let module_contents = include_bytes!("res/cases/no_mutants.wasm");

        let surviving_mutants = find_surviving_mutants(module_contents, &["no_mutants"], &[]);

        assert_eq!(surviving_mutants.len(), 0);
    }

    #[test]
    fn reports_no_surviving_mutants_when_all_mutants_killed() {
        let module_contents = include_bytes!("res/cases/no_surviving_mutants.wasm");

        let surviving_mutants =
            find_surviving_mutants(module_contents, &["no_surviving_mutants"], &[]);

        assert_eq!(surviving_mutants.len(), 0);
    }

    #[test]
    fn reports_surviving_mutant_when_mutant_not_killed() {
        let module_contents = include_bytes!("res/cases/surviving_mutants.wasm");

        let surviving_mutants =
            find_surviving_mutants(module_contents, &["surviving_mutants"], &[]);

        assert_eq!(surviving_mutants.len(), 1);
    }
}

mod candidates;
mod engine;
mod mutation;
mod runner;

fn main() {
    println!(
        "Original tests result: {:?}",
        runner::run_tests(include_bytes!("test-example.wasm"))
    );
    println!(
        "Surviving mutants: {:?}",
        engine::find_surviving_mutants(
            include_bytes!("test-example.wasm"),
            &vec!["roman_numerals"],
            &vec!["roman_numerals::tests"],
        )
    );
}

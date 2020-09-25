use wasmi::{ImportsBuilder, NopExternals, RuntimeValue};

pub fn run_tests(wasm_bytes: &[u8]) -> TestResult {
    let module = wasmi::Module::from_buffer(wasm_bytes).unwrap();
    let instance = wasmi::ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap()
        .assert_no_start();
    let result = instance.invoke_export(
        "main",
        &[RuntimeValue::from(0), RuntimeValue::from(0)],
        &mut NopExternals,
    );
    match result {
        Ok(_) => TestResult::Passed,
        Err(_) => TestResult::Failed,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TestResult {
    Passed,
    Failed,
}

#[cfg(test)]
mod tests {
    use crate::runner;
    use crate::runner::TestResult;

    #[test]
    fn runs_tests_which_pass() {
        let result = runner::run_tests(include_bytes!("./res/cases/test-example.wasm"));

        assert_eq!(result, TestResult::Passed);
    }

    #[test]
    fn runs_tests_which_fail() {
        let result = runner::run_tests(include_bytes!("./res/cases/test-example-failing.wasm"));

        assert_eq!(result, TestResult::Failed);
    }
}

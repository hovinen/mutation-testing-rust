use parity_wasm::elements::Module;
use rustc_demangle::demangle;

pub fn identify_candidate_functions(
    module: &Module,
    include_modules: &[&str],
    exclude_modules: &[&str],
) -> Vec<usize> {
    let mut result = Vec::<usize>::new();
    for function in module.names_section().unwrap().functions() {
        for name in function.names() {
            let demangled_name = demangle(name.1);
            for include_module in include_modules {
                if demangled_name.to_string().starts_with(include_module) {
                    let mut included = true;
                    for exclude_module in exclude_modules {
                        if demangled_name.to_string().starts_with(exclude_module) {
                            included = false;
                        }
                    }
                    if included {
                        result.push(name.0 as usize);
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::candidates::identify_candidate_functions;
    use parity_wasm::deserialize_buffer;
    use parity_wasm::elements::Module;
    use rustc_demangle::demangle;

    #[test]
    fn returns_no_functions_when_none_included() {
        let module = deserialize_buffer::<Module>(include_bytes!("res/cases/test-example.wasm"))
            .unwrap()
            .parse_names()
            .unwrap();

        let result = identify_candidate_functions(&module, &vec![], &vec![]);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn returns_functions_in_include_modules() {
        let module = deserialize_buffer::<Module>(include_bytes!("res/cases/test-example.wasm"))
            .unwrap()
            .parse_names()
            .unwrap();

        let result = identify_candidate_functions(&module, &vec!["roman_numerals"], &vec![]);

        let name_map = module.names_section().unwrap().functions().unwrap().names();
        let names: Vec<String> = result
            .into_iter()
            .map(|index| demangle(name_map.get(index as u32).unwrap()).to_string())
            .collect();
        let to_roman_name: Vec<String> = names
            .into_iter()
            .filter(|name| name.starts_with("roman_numerals::to_roman"))
            .collect();
        assert_eq!(to_roman_name.len(), 1);
    }

    #[test]
    fn does_not_return_functions_in_exclude_modules() {
        let module = deserialize_buffer::<Module>(include_bytes!("res/cases/test-example.wasm"))
            .unwrap()
            .parse_names()
            .unwrap();

        let result = identify_candidate_functions(
            &module,
            &vec!["roman_numerals"],
            &vec!["roman_numerals::tests"],
        );

        let name_map = module.names_section().unwrap().functions().unwrap().names();
        let names: Vec<String> = result
            .into_iter()
            .map(|index| demangle(name_map.get(index as u32).unwrap()).to_string())
            .collect();
        let to_roman_name: Vec<String> = names
            .into_iter()
            .filter(|name| name.starts_with("roman_numerals::tests"))
            .collect();
        assert_eq!(to_roman_name.len(), 0);
    }
}

use crate::model::variable::VariableSet;
use regex::Regex;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct VariableInjector {
    output_pattern: String,
}

impl VariableInjector {
    pub fn new(output_pattern: String) -> Self {
        Self { output_pattern }
    }

    pub(crate) fn inject(&self, variables: &VariableSet) -> String {
        let mut result = self.output_pattern.clone();
        variables.iter().for_each(|var| {
            result = result.replace(
                format!("<{}>", var.name.deref()).as_str(),
                var.value.as_str(),
            )
        });
        result = Regex::new("<.*>") // TODO: Replace with something more efficient
            .unwrap()
            .replace_all(result.as_str(), "")
            .to_string();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::{
        model::variable::{Variable, VariableSet},
        variable_mapping::VariableInjector,
    };

    #[test]
    fn inject_on_empty_pattern_with_empty_set_returns_empty_string() {
        let testee = VariableInjector::new("".to_string());

        let variables = VariableSet::empty();

        let result = testee.inject(&variables);

        assert_eq!(result, "")
    }

    #[test]
    fn inject_on_simple_pattern_with_empty_set_returns_empty_string() {
        let testee = VariableInjector::new("<x>/<y>".to_string());

        let variables = VariableSet::empty();

        let result = testee.inject(&variables);

        assert_eq!(result, "")
    }

    #[test]
    fn inject_on_simple_pattern_with_partial_variable_set_returns_partial_string() {
        let testee = VariableInjector::new("<x>/<y>".to_string());

        let variables = vec![Variable::new("x".into(), "hello".to_string())]
            .into_iter()
            .collect();

        let result = testee.inject(&variables);

        assert_eq!(result, "hello/")
    }

    #[test]
    fn fill_on_mult_pattern_with_all_variables_in_set_returns_full_string() {
        let testee = VariableInjector::new("<x>/<y>".to_string());

        let variables = vec![
            Variable::new("x".into(), "hello".to_string()),
            Variable::new("y".into(), "world".to_string()),
        ]
        .into_iter()
        .collect();

        let result = testee.inject(&variables);

        assert_eq!(result, "hello/world")
    }
}

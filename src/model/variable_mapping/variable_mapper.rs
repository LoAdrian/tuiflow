use regex::Regex;

use super::{VariableMapperCompilationError, VariableMappingError};

#[derive(Clone)]
pub(crate) struct VariableMapper {
    input_filter: Regex,
    output_format: String,
}

impl VariableMapper {
    pub fn new(
        input_filter_regex: &str,
        output_format: &str,
    ) -> Result<Self, VariableMapperCompilationError> {
        let input_filter = Regex::new(input_filter_regex);
        if input_filter.is_err() {
            return Err(VariableMapperCompilationError);
        }
        Ok(Self {
            input_filter: input_filter.unwrap(),
            output_format: String::from(output_format),
        })
    }

    pub fn map(&self, input: &str) -> impl Iterator<Item = Result<String, VariableMappingError>> {
        let input_capture_groups = self.input_filter.captures_iter(input);
        let input_filter_variable_names = self.input_filter.capture_names().skip(1);


        let iter = input_capture_groups
            .map(|input_capture_group| {
                let mut output: String = self.output_format.clone();
                for input_variable_name in input_filter_variable_names.clone() {
                    if let Some(variable_name) = input_variable_name {
                        // ignores unnamed capture groups
                        if let Some(variable_value) = input_capture_group.name(variable_name) {
                            output = output.replace(
                                format!("<{variable_name}>").as_str(),
                                variable_value.as_str(),
                            );
                        } else {
                            return Err(VariableMappingError);
                        }
                    }
                }
                Ok(output.clone())
            })
            .collect::<Vec<_>>();
        iter.into_iter()
    }
}

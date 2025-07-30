use super::VariableExtractor;
use crate::model::variable::{Variable, VariableName, VariableSet};
use tuiflow_model_contracts::error::VariableExtractorCompilationError;
use regex::{Captures, Regex};

#[derive(Clone)]
pub struct RegexVariableExtractor {
    input_filter: Regex,
}

impl RegexVariableExtractor {
    pub fn new(
        input_filter_regex: &str,
    ) -> Result<Self, VariableExtractorCompilationError> {
        let input_filter = Regex::new(input_filter_regex);
        match input_filter {
            Ok(regex) => Ok(Self {
                input_filter: regex,
            }),
            Err(_) => Err(VariableExtractorCompilationError(input_filter_regex.to_string())),
        }
    }
}

impl VariableExtractor for RegexVariableExtractor {
    fn extract(&self, input: &str) -> Vec<VariableSet> {
        let captures_matches = self.input_filter.captures_iter(input); // captures_per_match

        captures_matches
            .map(|captures_of_match: Captures| {
                self.input_filter.capture_names().skip(1)
                    .filter_map(|capture_group_name_opt| {
                        if let Some(capture_group_name) = capture_group_name_opt {
                            if let Some(variable_value) = captures_of_match.name(capture_group_name)
                            {
                                return Some(Variable {
                                    value: variable_value.as_str().to_string(),
                                    name: VariableName(capture_group_name.to_string()),
                                });
                            }
                        }
                        None
                    })
                    .collect::<VariableSet>()
            })
            .collect::<Vec<VariableSet>>()
    }
}

#[cfg(test)]
mod test {
    use crate::model::variable::Variable;
    use crate::variable_mapping::{RegexVariableExtractor, VariableExtractor};

    const FILE_NAME_EXTRACTOR_EXPRESSION: &str = "(?<base_path>.*)\\/(?<file_name>.*)\\.(?<file_ending>.*)";
    #[test]
    fn extract_with_file_name_extractor_extracts_variables() {
        let extractor = RegexVariableExtractor::new(FILE_NAME_EXTRACTOR_EXPRESSION)
            .unwrap();
        let file_paths = "/home/user/test.png\n/home/test.txt";

        let variables_sets = extractor.extract(file_paths);
        assert_eq!(variables_sets.len(), 2);
        assert!(variables_sets[0].contains(&Variable::new("base_path".into(), "/home/user".to_string())));
        assert!(variables_sets[0].contains(&Variable::new("file_name".into(), "test".to_string())));
        assert!(variables_sets[0].contains(&Variable::new("file_ending".into(), "png".to_string())));

        assert!(variables_sets[1].contains(&Variable::new("base_path".into(), "/home".to_string())));
        assert!(variables_sets[1].contains(&Variable::new("file_name".into(), "test".to_string())));
        assert!(variables_sets[1].contains(&Variable::new("file_ending".into(), "txt".to_string())));
    }

    #[test]
    fn extract_with_one_matching_and_one_unmatching_line_returns_matching_set() {
        let extractor = RegexVariableExtractor::new(FILE_NAME_EXTRACTOR_EXPRESSION)
            .unwrap();
        let file_paths = "/home/user/test.png\n/home/test,txt";

        let variables_sets = extractor.extract(file_paths);
        assert_eq!(variables_sets.len(), 1);
    }

    #[test]
    fn extract_with_empty_input_returns_empty_list() {
        let extractor = RegexVariableExtractor::new(FILE_NAME_EXTRACTOR_EXPRESSION)
            .unwrap();
        let file_paths = "";

        let variables_sets = extractor.extract(file_paths);
        assert_eq!(variables_sets.len(), 0);
    }
}
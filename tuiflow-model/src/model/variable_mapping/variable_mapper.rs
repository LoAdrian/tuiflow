use regex::Regex;

use super::{VariableMapper, VariableMapperCompilationError, VariableMappingError};

#[derive(Clone)]
pub struct RegexVariableMapper {
    input_filter: Regex,
    output_format: String,
}

impl RegexVariableMapper {
    pub fn new(
        input_filter_regex: &str,
        output_format: &str,
    ) -> Result<Self, VariableMapperCompilationError> {
        let input_filter = Regex::new(input_filter_regex);
        match input_filter {
            Ok(regex) => Ok(Self {
                input_filter: regex,
                output_format: String::from(output_format),
            }),
            Err(e) => Err(VariableMapperCompilationError(e)),
        }
    }

    pub fn identity() -> Self {
        Self {
            input_filter: Regex::new("(?<input>.*)")
                .expect("Failed to compile identity regex. Please report this on GitHub."),
            output_format: String::from("<input>"),
        }
    }
}

impl VariableMapper for RegexVariableMapper {
    fn map(&self, input: &str) -> impl Iterator<Item = Result<String, VariableMappingError>> {
        let input_capture_groups = self.input_filter.captures_iter(input);
        let input_filter_variable_names = self.input_filter.capture_names().skip(1); // skip whole match

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

    fn identity() -> Self {
        Self::new("(?<input>.*)", "<input>").unwrap_or_else(|_| Self::identity())
    }
}

#[cfg(test)]
mod variable_mapper_tests {
    use super::*;

    #[test]
    fn whitespace_mapper_example_works_without_error() {
        let extractor =
            RegexVariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
        let extracted = extractor.map("  world  ").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, world!");
    }

    #[test]
    fn mapper_example_with_variable_used_multiple_times_maps_variable_multiple_times() {
        let extractor = RegexVariableMapper::new(
            r"\s*(?<name>[^\s]+)\s*",
            r"Hello, <name>! Nice to meet you, <name>.",
        )
        .unwrap();
        let extracted = extractor.map("  charles  ").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, charles! Nice to meet you, charles.");
    }

    #[test]
    fn mapper_example_with_multiple_variables_maps_all_variables() {
        let extractor = RegexVariableMapper::new(
            r"(?<name1>.+),(?<name2>.+)",
            r"Hello, <name1>, brother of <name2>",
        )
        .unwrap();
        let extracted = extractor.map("luke,leia").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, luke, brother of leia");
    }

    #[test]
    fn mapper_example_with_multiple_variables_used_multiple_times_maps_all_variables_multiple_times(
    ) {
        let extractor = RegexVariableMapper::new(
            r"(?<name1>.+),(?<name2>.+)",
            r"Hello, <name1>, brother of <name2>. Hello <name2>, sister of <name1>.",
        )
        .unwrap();
        let extracted = extractor.map("leto,ghanima").nth(0).unwrap().unwrap();
        assert_eq!(
            extracted,
            "Hello, leto, brother of ghanima. Hello ghanima, sister of leto."
        );
    }

    #[test]
    fn whitespace_mapper_example_with_multiple_matches_works_correctly() {
        let extractor =
            RegexVariableMapper::new(r"\x20*(?<name>[^\x20\n]+)\x20*\n", r"Hello, <name>!")
                .unwrap();
        let extracted: Vec<Result<String, VariableMappingError>> =
            extractor.map("  mamfred \n charlie  \n").collect();
        assert_eq!(extracted[0].as_ref().unwrap(), "Hello, mamfred!");
        assert_eq!(extracted[1].as_ref().unwrap(), "Hello, charlie!");
    }

    #[test]
    fn mapper_ctor_with_invalid_regex_returns_err() {
        let extractor = RegexVariableMapper::new(r"[", r"Hello, <name>!");
        assert_eq!(true, extractor.is_err());
    }

    #[test]
    fn map_with_unmatching_input_returns_empty_iter() {
        let extractor =
            RegexVariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
        let mut extracted = extractor.map("   ");
        assert!(extracted.next().is_none());
    }

    #[test]
    fn map_with_matching_input_but_missing_variables_returns_err() {
        let extractor =
            RegexVariableMapper::new(r"\s*d(?<name>[abc])?\s*", r"Hello, <name>!").unwrap();
        let mut extracted = extractor.map(" d  ");
        assert!(extracted.nth(0).unwrap().is_err());
    }
}

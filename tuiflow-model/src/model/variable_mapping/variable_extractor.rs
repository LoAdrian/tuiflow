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
    fn parse(&self, input: &str) -> Vec<VariableSet> {
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

// #[cfg(test)]
// mod variable_mapper_tests {
//     use super::*;
//
//     #[test]
//     fn whitespace_mapper_example_works_without_error() {
//         let extractor =
//             RegexVariableExtractor::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
//         let extracted = extractor.parse("  world  ").nth(0).unwrap().unwrap();
//         assert_eq!(extracted, "Hello, world!");
//     }
//
//     #[test]
//     fn mapper_example_with_variable_used_multiple_times_maps_variable_multiple_times() {
//         let extractor = RegexVariableExtractor::new(
//             r"\s*(?<name>[^\s]+)\s*",
//             r"Hello, <name>! Nice to meet you, <name>.",
//         )
//             .unwrap();
//         let extracted = extractor.parse("  charles  ").nth(0).unwrap().unwrap();
//         assert_eq!(extracted, "Hello, charles! Nice to meet you, charles.");
//     }
//
//     #[test]
//     fn mapper_example_with_multiple_variables_maps_all_variables() {
//         let extractor = RegexVariableExtractor::new(
//             r"(?<name1>.+),(?<name2>.+)",
//             r"Hello, <name1>, brother of <name2>",
//         )
//             .unwrap();
//         let extracted = extractor.parse("luke,leia").nth(0).unwrap().unwrap();
//         assert_eq!(extracted, "Hello, luke, brother of leia");
//     }
//
//     #[test]
//     fn mapper_example_with_multiple_variables_used_multiple_times_maps_all_variables_multiple_times() {
//         let extractor = RegexVariableExtractor::new(
//             r"(?<name1>.+),(?<name2>.+)",
//             r"Hello, <name1>, brother of <name2>. Hello <name2>, sister of <name1>.",
//         )
//             .unwrap();
//         let extracted = extractor.parse("leto,ghanima").nth(0).unwrap().unwrap();
//         assert_eq!(
//             extracted,
//             "Hello, leto, brother of ghanima. Hello ghanima, sister of leto."
//         );
//     }
//
//     #[test]
//     fn whitespace_mapper_example_with_multiple_matches_works_correctly() {
//         let extractor =
//             RegexVariableExtractor::new(r"\x20*(?<name>[^\x20\n]+)\x20*\n", r"Hello, <name>!")
//                 .unwrap();
//         let extracted: Vec<Result<String, VariableMappingError>> =
//             extractor.parse("  mamfred \n charlie  \n").collect();
//         assert_eq!(extracted[0].as_ref().unwrap(), "Hello, mamfred!");
//         assert_eq!(extracted[1].as_ref().unwrap(), "Hello, charlie!");
//     }
//
//     #[test]
//     fn mapper_ctor_with_invalid_regex_returns_err() {
//         let extractor = RegexVariableExtractor::new(r"[", r"Hello, <name>!");
//         assert_eq!(true, extractor.is_err());
//     }
//
//     #[test]
//     fn map_with_unmatching_input_returns_empty_iter() {
//         let extractor =
//             RegexVariableExtractor::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
//         let mut extracted = extractor.parse("   ");
//         assert!(extracted.next().is_none());
//     }
//
//     #[test]
//     fn map_with_matching_input_but_missing_variables_returns_err() {
//         let extractor =
//             RegexVariableExtractor::new(r"\s*d(?<name>[abc])?\s*", r"Hello, <name>!").unwrap();
//         let mut extracted = extractor.parse(" d  ");
//         assert!(extracted.nth(0).unwrap().is_err());
//     }
// }

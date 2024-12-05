mod variable_mapper;
mod error;

pub(crate) use variable_mapper::VariableMapper;
pub(crate) use error::{VariableMapperCompilationError, VariableMappingError};

#[cfg(test)]
mod variable_mapper_tests {
    use super::*;

    #[test]
    fn whitespace_mapper_example_works_without_error() {
        let extractor = VariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
        let extracted = extractor.map("  world  ").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, world!");
    }

    #[test]
    fn mapper_example_with_variable_used_multiple_times_maps_variable_multiple_times() {
        let extractor = VariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>! Nice to meet you, <name>.").unwrap();
        let extracted = extractor.map("  charles  ").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, charles! Nice to meet you, charles.");
    }

    #[test]
    fn mapper_example_with_multiple_variables_maps_all_variables() {
        let extractor = VariableMapper::new(r"(?<name1>.+),(?<name2>.+)", r"Hello, <name1>, brother of <name2>").unwrap();
        let extracted = extractor.map("luke,leia").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, luke, brother of leia");
    }

    #[test]
    fn mapper_example_with_multiple_variables_used_multiple_times_maps_all_variables_multiple_times() {
        let extractor = VariableMapper::new(r"(?<name1>.+),(?<name2>.+)", r"Hello, <name1>, brother of <name2>. Hello <name2>, sister of <name1>.").unwrap();
        let extracted = extractor.map("leto,ghanima").nth(0).unwrap().unwrap();
        assert_eq!(extracted, "Hello, leto, brother of ghanima. Hello ghanima, sister of leto.");
    }

    #[test]
    fn whitespace_mapper_example_with_multiple_matches_works_correctly() {
        let extractor = VariableMapper::new(r"\x20*(?<name>[^\x20\n]+)\x20*\n", r"Hello, <name>!").unwrap();
        let extracted: Vec<Result<String, VariableMappingError>> = extractor.map("  mamfred \n charlie  \n").collect();
        assert_eq!(extracted[0].as_ref().unwrap(), "Hello, mamfred!");
        assert_eq!(extracted[1].as_ref().unwrap(), "Hello, charlie!");
    }

    #[test]
    fn mapper_ctor_with_invalid_regex_returns_err() {
        let extractor = VariableMapper::new(r"[", r"Hello, <name>!");
        assert_eq!(true, extractor.is_err());
    }

    #[test]
    fn map_with_unmatching_input_returns_empty_iter() {
        let extractor = VariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
        let mut extracted = extractor.map("   ");
        assert!(extracted.next().is_none());
    }

    #[test]
    fn map_with_matching_input_but_missing_variables_returns_err() {
        let extractor = VariableMapper::new(r"\s*d(?<name>[abc])?\s*", r"Hello, <name>!").unwrap();
        let mut extracted = extractor.map(" d  ");
        assert!(extracted.nth(0).unwrap().is_err());
    }
}
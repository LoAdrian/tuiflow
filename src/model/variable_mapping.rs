mod variable_mapper;
mod error;

pub use variable_mapper::VariableMapper;
pub use error::{VariableMapperCompilationError, VariableMappingError};

#[cfg(test)]
mod variable_mapper_tests {
    use super::*;

    #[test]
    fn whitespace_mapper_example_works_without_error() {
        let extractor = VariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
        let extracted = extractor.map("  world  ").unwrap();
        assert_eq!(extracted, "Hello, world!");
    }

    #[test]
    fn mapper_ctor_with_invalid_regex_returns_err() {
        let extractor = VariableMapper::new(r"[", r"Hello, <name>!");
        assert_eq!(true, extractor.is_err());
    }

    #[test]
    fn map_with_unmatching_input_returns_err() {
        let extractor = VariableMapper::new(r"\s*(?<name>[^\s]+)\s*", r"Hello, <name>!").unwrap();
        let extracted = extractor.map("   ");
        assert_eq!(VariableMappingError::InputParsingFailed, extracted.err().unwrap());
    }
    #[test]
    fn map_with_matching_input_but_missing_variables_returns_err() {
        let extractor = VariableMapper::new(r"\s*d(?<name>[abc])?\s*", r"Hello, <name>!").unwrap();
        let extracted = extractor.map(" d  ");
        assert_eq!(VariableMappingError::VariableMappingFailed, extracted.err().unwrap());
    }
}
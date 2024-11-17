use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct VariableMapperCompilationError;
impl Display for VariableMapperCompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input filter regex compilation failed. Make sure that it complies with: https://docs.rs/regex/latest/regex/#syntax")
    }
}

#[derive(Debug, PartialEq)]
pub enum VariableMappingError {
    InputParsingFailed,
    VariableMappingFailed
}

impl Display for VariableMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableMappingError::InputParsingFailed => write!(f, "Could not parse input with input filter regex"),
            VariableMappingError::VariableMappingFailed => write!(f, "Could not map input variables to output. Make sure that all expected variables are present in the input.")
        }
    }
}
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub(crate) struct VariableMapperCompilationError;
impl Display for VariableMapperCompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input filter regex compilation failed. Make sure that it complies with: https://docs.rs/regex/latest/regex/#syntax")
    }
}

// TODO: Add more info to error
#[derive(Debug, PartialEq)]
pub(crate) struct VariableMappingError;

impl Display for VariableMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Could not map input variables to output. Make sure that all expected variables are present in the input.")
    }
}
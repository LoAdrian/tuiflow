use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::command_runner::CommandRunnerError;
use crate::control::Key;
#[derive(PartialEq, Debug)]
pub enum StateTransitionError {
    VariableMappingError(VariableMappingError),
    ControlNotFound(Key),
    CommandExecutionError(CommandRunnerError),
}

impl Display for StateTransitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StateTransitionError::VariableMappingError(e) => {
                write!(f, "Mapping variables failed: {e}")
            }
            StateTransitionError::ControlNotFound(control) => {
                write!(f, "Control not found: {control}")
            }
            StateTransitionError::CommandExecutionError(command) => {
                write!(f, "CLI command execution failed: {command}")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableExtractorCompilationError(pub String);
impl Display for VariableExtractorCompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input filter regex ({}) compilation failed. Make sure that it complies with: https://docs.rs/regex/latest/regex/#syntax", self.0)
    }
}

impl Error for VariableExtractorCompilationError {}

// TODO: Add more info to error
#[derive(Debug, PartialEq, Clone)]
pub struct VariableMappingError;

impl Display for VariableMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not map input variables to output. Make sure that all expected variables are present in the input.")
    }
}

impl Error for VariableMappingError {}

#[derive(Debug)]
pub struct InitialTransitionError(StateTransitionError);

impl From<StateTransitionError> for InitialTransitionError {
    fn from(value: StateTransitionError) -> Self {
        InitialTransitionError(value)
    }
}

impl Display for InitialTransitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initial transition error: {}", self.0)
    }
}

impl Error for InitialTransitionError {}

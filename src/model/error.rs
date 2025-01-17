use std::fmt::Display;

use super::{control::Control, variable_mapping::VariableMappingError};

#[derive(PartialEq, Debug)]
pub enum StateTransitionError {
    SelectionToCommandMappingFailed(VariableMappingError),
    ControlNotFound(Control),
    CliCommandExecutionFailed(String),
}

impl Display for StateTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateTransitionError::SelectionToCommandMappingFailed(e) => {
                write!(f, "Selection to command mapping failed: {e}")
            }
            StateTransitionError::ControlNotFound(control) => {
                write!(f, "Control not found: {control}")
            }
            StateTransitionError::CliCommandExecutionFailed(command) => {
                write!(f, "CLI command execution failed: {command}")
            }
        }
    }
}

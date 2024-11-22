use std::fmt::Display;

use super::variable_mapping::VariableMappingError;

pub enum StateTransitionError  {
    SelectionToCommandMappingFailed(VariableMappingError),
    ControlNotFound(char),
    CliCommandExecutionFailed(String)
}

impl Display for StateTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
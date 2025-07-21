use crate::model::variable_mapping::{VariableMapper, VariableMappingError};
use std::{cell::RefCell, fmt::Display, rc::Rc};
use tuiflow_model_contracts::command_runner::CommandRunner;
use super::{control::Control, state::State};

pub mod builder;

pub struct Transition<R: CommandRunner, M: VariableMapper> {
    control: Control,
    next_state: Rc<RefCell<State<R, M>>>, //TODO: Check and break cycles
    selected_display_to_command: M,       // regex extraction from selection
}

impl<R: CommandRunner, M: VariableMapper> Transition<R, M> {
    pub fn new(
        control: Control,
        next_state: Rc<RefCell<State<R, M>>>,
        selected_display_to_cmd: M,
    ) -> Self {
        Self {
            control,
            next_state,
            selected_display_to_command: selected_display_to_cmd,
        }
    }

    pub fn get_transition_command(
        &self,
        input: Option<&str>,
    ) -> Result<R::Command, DisplayToCommandMappingError> {
        let result = self
            .selected_display_to_command
            .map(input.map_or("", |s| s))
            .nth(0);

        match result {
            Some(Ok(command)) => Ok(command.into()),
            Some(Err(e)) => Err(DisplayToCommandMappingError::VariableMappingError(e)),
            None => Err(DisplayToCommandMappingError::NoMatchFound),
        }
    }

    pub fn get_activation_control(&self) -> &Control {
        &self.control
    }

    pub fn get_next_state(&self) -> Rc<RefCell<State<R, M>>> {
        Rc::clone(&self.next_state)
    }
}

impl<R: CommandRunner, M: VariableMapper> Clone for Transition<R, M> {
    fn clone(&self) -> Self {
        Self {
            control: self.control.clone(),
            next_state: Rc::clone(&self.next_state),
            selected_display_to_command: self.selected_display_to_command.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayToCommandMappingError {
    VariableMappingError(VariableMappingError),
    NoMatchFound,
}

impl Display for DisplayToCommandMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayToCommandMappingError::VariableMappingError(e) => {
                write!(f, "Variable mapping error: {e}")
            }
            DisplayToCommandMappingError::NoMatchFound => {
                write!(f, "No match found for the display selection")
            }
        }
    }
}
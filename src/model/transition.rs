use std::{cell::RefCell, rc::Rc};

use mockall::predicate::ne;

use crate::model::variable_mapping::{VariableMapper, VariableMappingError};

use super::{
    control::Control,
    state::State, workflow::CommandRunner, Line,
};

pub(crate) mod builder;

pub(crate) struct Transition<R: CommandRunner, M: VariableMapper> {
    control: Control,
    next_state: Rc<RefCell<State<R, M>>>,    //TODO: Check and break cycles
    selected_display_to_command: M, // regex extraction from selection
}

impl<R: CommandRunner, M: VariableMapper> Transition<R, M> {
    pub fn new(control: Control, next_state: Rc<RefCell<State<R, M>>>, selected_display_to_cmd: M) -> Self {
        Self {
            control,
            next_state,
            selected_display_to_command: selected_display_to_cmd,
        }
    }

    pub fn get_transition_command(
        &self,
        selected_line: &Line,
    ) -> Result<String, VariableMappingError> {
        self.selected_display_to_command
            .map(&selected_line.0)
            .nth(0)
            .unwrap()
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

/*
#[cfg(test)]
mod transition_tests {
    use std::rc::Rc;

    use crate::{model::{control::Key, variable_mapping::MockVariableMapper, Control}, workflow::MockCommandRunner};

    use super::{State, Transition};

    #[test]
    fn get_next_state_returns_copied_reference_to_original_state() {
        // Arrange
        let control = Control::new("test_control", Key::Char('a'));
        let command_output_to_display = MockVariableMapper::new();
        let selected_display_to_command = MockVariableMapper::new();
        let original_state = Rc::new(State::new(
            "test_state",
            command_output_to_display,
        ));
        let transition = Transition::new(
            control,
            Rc::clone(&original_state),
            selected_display_to_command,
        );

        // Act
        let next_state = transition.get_next_state();

        // Assert
        assert!(Rc::ptr_eq(&next_state, &original_state));
    }
}
*/
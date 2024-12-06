use std::rc::Rc;

use crate::model::variable_mapping::{VariableMapper, VariableMappingError};

use super::{
    control::Control,
    state::{State, StateContext},
    variable_mapping::RegexVariableMapper,
};

pub(crate) struct Transition<C: StateContext<M>, M: VariableMapper> {
    control: Control,
    next_state: Rc<State<C, M>>,    //TODO: Check and break cycles
    selected_display_to_command: M, // regex extraction from selection
}

impl<C: StateContext<M>, M: VariableMapper> Transition<C, M> {
    pub fn new(control: Control, next_state: Rc<State<C, M>>, selected_display_to_cmd: M) -> Self {
        Self {
            control,
            next_state,
            selected_display_to_command: selected_display_to_cmd,
        }
    }

    pub fn get_transition_command(
        &self,
        display_selection: &str,
    ) -> Result<String, VariableMappingError> {
        self.selected_display_to_command
            .map(display_selection)
            .nth(0)
            .unwrap()
    }

    pub fn get_activation_control(&self) -> &Control {
        &self.control
    }

    pub fn get_next_state(&self) -> Rc<State<C, M>> {
        Rc::clone(&self.next_state)
    }
}

impl<C: StateContext<M>, M: VariableMapper> Clone for Transition<C, M> {
    fn clone(&self) -> Self {
        Self {
            control: self.control.clone(),
            next_state: Rc::clone(&self.next_state),
            selected_display_to_command: self.selected_display_to_command.clone(),
        }
    }
}

#[cfg(test)]
mod transition_tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::model::{state::MockStateContext, variable_mapping::MockVariableMapper, Control};

    use super::{RegexVariableMapper, State, Transition};

    #[test]
    fn get_next_state_returns_copied_reference_to_original_state() {
        // Arrange
        let context = MockStateContext::new();
        let control = Control::new("test_control", "c");
        let command_output_to_display = MockVariableMapper::new();
        let selected_display_to_command = MockVariableMapper::new();
        let original_state = Rc::new(State::new(
            "test_state",
            command_output_to_display,
            Rc::new(RefCell::new((context))),
            vec![],
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

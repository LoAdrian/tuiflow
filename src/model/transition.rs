use std::rc::Rc;

use crate::model::variable_mapping::{VariableMapper, VariableMappingError};

use super::state::{State, StateContext};

pub(crate) struct Transition<C: StateContext> {
    control: Control,
    next_state: Rc<State<C>>,
    selected_display_to_cmd: VariableMapper, // regex extraction from selection
}

impl<C: StateContext> Transition<C> {
    pub fn new(control: Control, next_state: &Rc<State<C>>, selected_display_to_cmd: VariableMapper) -> Self {
        Self {
            control,
            next_state: Rc::clone(next_state),
            selected_display_to_cmd,
        }
    }

    pub fn get_transition_command(&self, display_selection: &str) -> Result<String, VariableMappingError> {
        self.selected_display_to_cmd.map(display_selection)
    }
    
    pub fn get_activation_control(&self) -> &Control {
        &self.control
    }

    pub fn get_next_state(&self) -> Rc<State<C>> {
        Rc::clone(&self.next_state)
    }
}

pub(crate) struct Control {
    pub name: String,
    pub key: char // probably should not be char
}
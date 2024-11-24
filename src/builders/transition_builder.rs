use std::rc::Rc;

use crate::model::{state::{State, StateContext}, transition::Transition, variable_mapping::VariableMapper, Control};

pub struct TransitionBuilder<C: StateContext> {
    control: Option<Control>,
    next_state: Option<Rc<State<C>>>,
    selected_display_to_command: Option<VariableMapper>,
}

impl<C: StateContext> Default for TransitionBuilder<C> {
    fn default() -> Self {
        Self {
            control: Default::default(),
            next_state: Default::default(),
            selected_display_to_command: Default::default(),
        }
    }
}

impl<C: StateContext> TransitionBuilder<C> {
    pub fn new() -> Self { Default::default() }

    pub fn with_control(mut self, control: Control) -> Self {
        self.control = Some(control);
        self
    }

    pub fn with_next_state(mut self, next_state: &Rc<State<C>>) -> Self {
        self.next_state = Some(Rc::clone(next_state));
        self
    }

    pub fn with_selected_display_to_command(mut self, selected_display_to_command: VariableMapper) -> Self {
        self.selected_display_to_command = Some(selected_display_to_command);
        self
    }

    pub fn build(self) -> Transition<C> {
        Transition::new(
            self.control.expect("Control is required to build Transition"),
            self.next_state.expect("Next state is required to build Transition"),
            self.selected_display_to_command.expect("Selected display to command is required to build Transition"),
        )
    }
}

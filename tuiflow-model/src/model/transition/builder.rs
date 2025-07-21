use std::{cell::RefCell, rc::Rc};
use tuiflow_model_contracts::command_runner::CommandRunner;
use crate::Control;
use crate::state::State;
use crate::transition::Transition;
use crate::variable_mapping::VariableMapper;

pub struct TransitionBuilder<R: CommandRunner, M: VariableMapper> {
    control: Option<Control>,
    next_state: Option<Rc<RefCell<State<R, M>>>>,
    selected_display_to_command: Option<M>,
}

impl<R: CommandRunner, M: VariableMapper> Default for TransitionBuilder<R, M> {
    fn default() -> Self {
        Self {
            control: Default::default(),
            next_state: Default::default(),
            selected_display_to_command: Default::default(),
        }
    }
}

impl<R: CommandRunner, M: VariableMapper> TransitionBuilder<R, M> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_control(&mut self, control: Control) -> &mut Self {
        self.control = Some(control);
        self
    }

    pub fn with_next_state(&mut self, next_state: Rc<RefCell<State<R, M>>>) -> &mut Self {
        self.next_state = Some(next_state);
        self
    }

    pub fn with_selected_display_to_command(
        &mut self,
        selected_display_to_command: M,
    ) -> &mut Self {
        self.selected_display_to_command = Some(selected_display_to_command);
        self
    }

    pub fn build(&self) -> Transition<R, M> {
        Transition::new(
            self.control
                .clone()
                .expect("Control is required to build Transition"),
            self.next_state
                .clone()
                .expect("Next state is required to build Transition"),
            self.selected_display_to_command
                .clone()
                .expect("Selected display to command is required to build Transition"),
        )
    }
}

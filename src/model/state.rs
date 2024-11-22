use std::{cell::RefCell, collections::HashMap, rc::Rc};
use super::{error::StateTransitionError, transition::Transition, variable_mapping::VariableMapper};

pub(crate) struct State<C: StateContext> {
    display_name: String,
    command_output_to_display: VariableMapper,
    transitions: HashMap<char, Transition<C>>,
    context: Rc<RefCell<C>>,
}

impl<C: StateContext> State<C> {
    pub fn new(display_name: &str, command_output_to_display: VariableMapper, transitions: Vec<Transition<C>>, context: Rc<RefCell<C>>) -> Self {
        let transition_mapping = transitions
            .into_iter()
            .map(|t| (t.get_activation_control().key, t))
            .collect::<HashMap<_, _>>();

        Self {
            display_name: String::from(display_name),
            command_output_to_display,
            transitions: transition_mapping,
            context
        }
    }

    pub fn transition(&self, display_selection: &str, control: char) -> Result<(), StateTransitionError> {
        if let Some(transition) = self.transitions.get(&control) {
            let transition_command = transition.get_transition_command(display_selection);
            if let Ok(command_to_execute) = transition_command {
                let next_state = transition.get_next_state();
                let mut context = self.context.borrow_mut();
                context.update(next_state, command_to_execute)
            } else {
                let transition_error = transition_command.unwrap_err();
                Err(StateTransitionError::SelectionToCommandMappingFailed(transition_error))
            }
        } else {
            Err(StateTransitionError::ControlNotFound(control))
        }
    }
}

pub(crate) trait StateContext : Sized {
    fn update(&mut self, state: Rc<State<Self>>, command: String) -> Result<(), StateTransitionError>;
}
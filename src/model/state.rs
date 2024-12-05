use std::{cell::RefCell, collections::HashMap, rc::Rc};
use super::{control::Control, display::{Display, Line}, error::StateTransitionError, transition::Transition, variable_mapping::VariableMapper};

#[derive(Clone)]
pub(crate) struct State<C: StateContext> {
    display_name: String,
    command_output_to_display: VariableMapper,
    transitions: HashMap<String, Transition<C>>,
    context: Rc<RefCell<C>>,
}

impl<C: StateContext> State<C> {
    pub(crate) fn new(
        display_name: &str, 
        command_output_to_display: VariableMapper,
        context: Rc<RefCell<C>>,
        transitions: Vec<Transition<C>>) -> Self {
        let transition_mapping = transitions
            .into_iter()
            .map(|t: Transition<C>| (String::from(t.get_activation_control().get_key()), t))
            .collect::<HashMap<String, Transition<C>>>();

        Self {
            display_name: String::from(display_name),
            command_output_to_display,
            transitions: transition_mapping,
            context,
        }
    }

    pub(crate) fn add_transition(&mut self, transition: Transition<C>) {
        self.transitions.insert(String::from(transition.get_activation_control().get_key()), transition);
    }

    pub(crate) fn transition(&self, display_selection: &str, control: Control) -> Result<(), StateTransitionError> {
        if let Some(transition) = self.transitions.get(control.get_key()) {
            let transition_command = transition.get_transition_command(display_selection);
            if let Ok(command_to_execute) = transition_command {
                let next_state = transition.get_next_state();
                let mut context = self.context.borrow_mut();
                let cli_result = context.run_command(&command_to_execute);
                if let Ok(cli_output)  = cli_result {
                    let display = self.parse_display(&cli_output);
                    context.update(next_state, display);
                    Ok(())
                } else {
                    Err(StateTransitionError::CliCommandExecutionFailed(command_to_execute.clone()))
                }

            } else {
                let transition_error = transition_command.unwrap_err();
                Err(StateTransitionError::SelectionToCommandMappingFailed(transition_error))
            }
        } else {
            Err(StateTransitionError::ControlNotFound(control))
        }
    }
    
    fn parse_display(&self, command_output: &str) -> Display {
        let mut errors = Vec::new();
        let mut lines = Vec::new();
        for line_result in self.command_output_to_display.map(command_output) {
            match line_result {
                Ok(line) => lines.push(Line(line)),
                Err(e) => errors.push(format!("{e}"))
            }
        }

        Display {
            lines,
            errors,
            ..Default::default()
        }
    }
}

pub(crate) trait StateContext : Sized {
    fn update(&mut self, state: Rc<State<Self>>, display: Display);
    fn run_command(&self, command: &str) -> Result<String, ()>;
}
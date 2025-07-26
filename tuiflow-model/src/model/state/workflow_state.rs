use tuiflow_model_contracts::control::Key;
use tuiflow_model_contracts::display::DisplayError;
use tuiflow_model_contracts::error::StateTransitionError;
use tuiflow_model_contracts::error::StateTransitionError::ControlNotFound;
use crate::model::variable::VariableSet;
use crate::state::state::State;
use crate::transition::Transition;
use crate::variable_mapping::VariableInjector;
use crate::variable_mapping::VariableExtractor;
use crate::{Control, Display};
use std::collections::HashMap;
use tuiflow_model_contracts::command_runner::CommandRunner;

#[derive(Clone)]
pub struct WorkflowState<R: CommandRunner, M: VariableExtractor> {
    display_name: String,
    command_output_to_display: VariableInjector,
    transitions: HashMap<Key, Transition<R, M>>,
}

impl<R: CommandRunner, M: VariableExtractor> WorkflowState<R, M> {
    pub fn new(
        display_name: &str,
        display_variable_injector: VariableInjector,
        transitions: Vec<Transition<R, M>>,
    ) -> Self {
        let transition_mapping = transitions
            .into_iter()
            .map(|t: Transition<R, M>| (t.get_activation_control().get_key(), t))
            .collect::<HashMap<Key, Transition<R, M>>>();

        Self {
            display_name: String::from(display_name),
            command_output_to_display: display_variable_injector,
            transitions: transition_mapping,
        }
    }

    pub(crate) fn transition(
        &self,
        selected_variable_set: &VariableSet,
        key: &Key,
    ) -> Result<State<R, M>, StateTransitionError> {
        if let Some(transition) = self.transitions.get(key) {
            transition.run(selected_variable_set)
        } else {
            Err(ControlNotFound(*key))
        }
    }

    pub(crate) fn get_controls(&self) -> Vec<Control> {
        self.transitions
            .iter()
            .map(|(_, transition)| transition.get_activation_control().clone())
            .collect()
    }

    pub fn add_transition(&mut self, key: Key, transition: Transition<R, M>) {
        self.transitions.insert(key, transition);
    }

    pub(crate) fn get_display(&self, variable_set: &Vec<VariableSet>) -> Display {
        let lines = variable_set
            .iter()
            .filter_map(|set| {
                self.command_output_to_display
                    .inject(set)
                    .ok()
                    .map(|content| content.into())
            })
            .collect();

        let errors = variable_set
            .iter()
            .filter_map(|set| {
                self.command_output_to_display
                    .inject(set)
                    .err()
                    .map(|err| DisplayError(format!("Error: {}", err)))
            })
            .collect();

        Display { lines, errors }
    }

    pub(crate) fn get_display_name(&self) -> String {
        self.display_name.clone()
    }
}

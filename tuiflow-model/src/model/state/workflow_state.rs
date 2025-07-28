use crate::model::variable::VariableSet;
use crate::state::state::State;
use crate::state::Transit;
use crate::variable_mapping::VariableInjector;
use crate::{Control, Display};
use std::collections::HashMap;
use tuiflow_model_contracts::control::Key;
use tuiflow_model_contracts::display::DisplayError;
use tuiflow_model_contracts::error::StateTransitionError;
use tuiflow_model_contracts::error::StateTransitionError::ControlNotFound;

#[derive(Clone)]
pub struct WorkflowState<T: Transit> {
    display_name: String,
    command_output_to_display: VariableInjector,
    transitions: HashMap<Key, T>,
}

impl<T: Transit> WorkflowState<T> {
    pub fn new(
        display_name: &str,
        display_variable_injector: VariableInjector,
        transitions: Vec<T>,
    ) -> Self {
        let transition_mapping = transitions
            .into_iter()
            .map(|t: T| (t.get_activation_control().get_key(), t))
            .collect::<HashMap<Key, T>>();

        Self {
            display_name: String::from(display_name),
            command_output_to_display: display_variable_injector,
            transitions: transition_mapping,
        }
    }

    pub fn add_transition(&mut self, key: Key, transition: T) {
        self.transitions.insert(key, transition);
    }

    pub(crate) fn transition(
        &self,
        selected_variable_set: &VariableSet,
        key: &Key,
    ) -> Result<State<T>, StateTransitionError> {
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

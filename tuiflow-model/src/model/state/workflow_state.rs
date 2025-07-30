use crate::model::variable::VariableSet;
use crate::state::state::State;
use crate::state::Transit;
use crate::variable_mapping::VariableInjector;
use crate::{Control, Display};
use std::collections::HashMap;
use tuiflow_model_contracts::control::Key;
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
            .map(|set| {
                self.command_output_to_display
                    .inject(set)
                    .into()
            })
            .collect();

        Display { lines }
    }

    pub(crate) fn get_display_name(&self) -> String {
        self.display_name.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{MockTransit, State, WorkflowState};
    use crate::variable_mapping::VariableInjector;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tuiflow_model_contracts::control::{Control, Key};
    use crate::model::variable::VariableSet;

    #[test]
    fn transition_with_existing_control_runs_transitions() {
        let mut mock_transition = MockTransit::new();
        let target_state_display_name = "target state";
        mock_transition
            .expect_run()
            .once()
            .returning(move |_| {
                let variable_injector = VariableInjector::new("some pattern".to_string());
                let target_state = WorkflowState::new(target_state_display_name, variable_injector.clone(), vec![]);
                Ok(State::new(Rc::new(RefCell::new(target_state)), vec![]))
            });

        let activation_control = Control::new("some control", Key::Esc);
        mock_transition
            .expect_get_activation_control()
            .return_const(activation_control.clone());

        let variable_injector = VariableInjector::new("some pattern".to_string());
        let testee = WorkflowState::new("some state", variable_injector, vec![mock_transition]);
        let target_state = testee.transition(&VariableSet::empty(), &activation_control.get_key());
        assert!(target_state.is_ok());
        assert_eq!(target_state.unwrap().get_name().as_str(), target_state_display_name);
    }

}
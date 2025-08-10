use crate::model::variable::VariableSet;
use crate::state::state::State;
use crate::state::Transit;
use crate::variable_mapping::VariableInjector;
use crate::{Control, Display};
use std::collections::HashMap;
use tuiflow_model_contracts::control::Key;
use tuiflow_model_contracts::error::StateTransitionError;
use tuiflow_model_contracts::error::StateTransitionError::{
    ControlNotFound, NoMatchingTransitionCommandPattern,
};

#[derive(Clone)]
pub struct WorkflowState<T: Transit> {
    display_name: String,
    command_output_to_display: VariableInjector,
    transitions: HashMap<Key, Vec<T>>,
}

impl<T: Transit> WorkflowState<T> {
    pub fn new(
        display_name: &str,
        display_variable_injector: VariableInjector,
        transitions: Vec<T>,
    ) -> Self {
        let transition_mapping: HashMap<Key, Vec<T>> = HashMap::new();

        let mut _self = Self {
            display_name: String::from(display_name),
            command_output_to_display: display_variable_injector,
            transitions: transition_mapping,
        };
        for transition in transitions {
            let key = transition.get_activation_control().get_key();
            _self.add_transition(key, transition);
        }

        _self
    }

    pub fn add_transition(&mut self, key: Key, transition: T) {
        if let Some(existing_transitions) = self.transitions.get_mut(&key) {
            existing_transitions.push(transition);
            return;
        } else {
            self.transitions.insert(key, vec![transition]);
        }
    }

    pub(crate) fn transition(
        &self,
        selected_variable_set: &VariableSet,
        key: &Key,
    ) -> Result<State<T>, StateTransitionError> {
        if let Some(transitions) = self.transitions.get(key) {
            for transition in transitions {
                if let Some(result) = transition.run(selected_variable_set) {
                    return result;
                }
            }
            Err(NoMatchingTransitionCommandPattern)
        } else {
            Err(ControlNotFound(*key))
        }
    }

    pub(crate) fn get_controls(&self) -> Vec<Control> {
        self.transitions
            .iter()
            .map(|(_, transition)| {
                transition
                    .first()
                    .expect("Key was added to list of transitions without any transition.")
                    .get_activation_control()
                    .clone()
            })
            .collect()
    }

    pub(crate) fn get_display(&self, variable_set: &Vec<VariableSet>) -> Display {
        let lines = variable_set
            .iter()
            .map(|set| self.command_output_to_display.inject(set).unwrap().into()) // TODO: Handle error
            .collect();

        Display { lines }
    }

    pub(crate) fn get_display_name(&self) -> String {
        self.display_name.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::variable::{Variable, VariableSet};
    use crate::state::{MockTransit, State, WorkflowState};
    use crate::variable_mapping::VariableInjector;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tuiflow_model_contracts::control::{Control, Key};
    use tuiflow_model_contracts::error::StateTransitionError;

    #[test]
    fn transition_with_existing_control_runs_transitions() {
        let target_state_display_name = "target state";
        let activation_control = Control::new("some control", Key::Esc);
        let mock_transition = get_mock_transition(
            target_state_display_name,
            "some_pattern",
            true,
            activation_control.clone(),
        );

        let variable_injector = VariableInjector::new("some pattern".to_string());
        let testee = WorkflowState::new("some state", variable_injector, vec![mock_transition]);
        let target_state = testee.transition(&VariableSet::empty(), &activation_control.get_key());
        assert!(target_state.is_ok());
        assert_eq!(
            target_state.unwrap().get_name().as_str(),
            target_state_display_name
        );
    }

    #[test]
    fn run_with_control_with_multiple_transitions_picks_first_if_both_patterns_are_filled() {
        let target_state_display_name = "target state";
        let activation_control = Control::new("some control", Key::Esc);

        let target_transition = get_mock_transition(
            target_state_display_name,
            "<x>/<y>",
            true,
            activation_control.clone(),
        );
        let second_transition =
            get_mock_transition("second state", "<y>/<x>", true, activation_control.clone());

        let variable_injector = VariableInjector::new("some pattern".to_string());
        let testee = WorkflowState::new(
            "some state",
            variable_injector,
            vec![target_transition, second_transition],
        );
        let variable_set = vec![
            Variable::new("x".into(), "hello".to_string()),
            Variable::new("y".into(), "world".to_string()),
        ]
        .into_iter()
        .collect();
        let target_state = testee.transition(&variable_set, &activation_control.get_key());
        assert!(target_state.is_ok());
        assert_eq!(
            target_state.unwrap().get_name().as_str(),
            target_state_display_name
        );
    }

    #[test]
    fn run_with_control_with_multiple_transitions_picks_second_if_first_does_not_match() {
        let target_state_display_name = "target state";
        let activation_control = Control::new("some control", Key::Esc);

        let first_transition = get_mock_transition(
            "first state",
            "<x>/<y>/<z>",
            false,
            activation_control.clone(),
        );
        let second_transition = get_mock_transition(
            target_state_display_name,
            "<y>/<x>",
            true,
            activation_control.clone(),
        );

        let variable_injector = VariableInjector::new("some pattern".to_string());
        let testee = WorkflowState::new(
            "some state",
            variable_injector,
            vec![first_transition, second_transition],
        );
        let variable_set = vec![
            Variable::new("x".into(), "hello".to_string()),
            Variable::new("y".into(), "world".to_string()),
        ]
        .into_iter()
        .collect();
        let target_state = testee.transition(&variable_set, &activation_control.get_key());
        assert!(target_state.is_ok());
        assert_eq!(
            target_state.unwrap().get_name().as_str(),
            target_state_display_name
        );
    }

    #[test]
    fn run_with_control_with_multiple_transitions_returns_error_if_both_patterns_unfilled() {
        let activation_control = Control::new("some control", Key::Esc);

        let first_transition = get_mock_transition(
            "first state",
            "<x>/<y>/<z>",
            false,
            activation_control.clone(),
        );
        let second_transition = get_mock_transition(
            "second state",
            "<y>/<x>/<z>",
            false,
            activation_control.clone(),
        );

        let variable_injector = VariableInjector::new("some pattern".to_string());
        let testee = WorkflowState::new(
            "some state",
            variable_injector,
            vec![first_transition, second_transition],
        );
        let variable_set = vec![
            Variable::new("x".into(), "hello".to_string()),
            Variable::new("y".into(), "world".to_string()),
        ]
        .into_iter()
        .collect();
        let target_state = testee.transition(&variable_set, &activation_control.get_key());
        assert!(target_state.is_err());
        assert_eq!(
            target_state.err().unwrap(),
            StateTransitionError::NoMatchingTransitionCommandPattern
        );
    }

    fn get_mock_transition(
        target_state_display_name: &'static str,
        command_builder_pattern: &'static str,
        matches_input: bool,
        activation_control: Control,
    ) -> MockTransit {
        let mut transition = MockTransit::new();
        transition
            .expect_run()
            .times(0..=1)
            .withf(|_| true)
            .returning(move |_| {
                let variable_injector = VariableInjector::new(command_builder_pattern.to_string());
                let target_state = WorkflowState::new(
                    target_state_display_name,
                    variable_injector.clone(),
                    vec![],
                );
                if matches_input {
                    Some(Ok(State::new(Rc::new(RefCell::new(target_state)), vec![])))
                } else {
                    None
                }
            });

        transition
            .expect_get_activation_control()
            .times(0..=1)
            .return_const(activation_control);
        transition
    }
}

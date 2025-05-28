use std::{cell::{BorrowMutError, RefCell}, collections::HashMap, rc::Rc};

use super::{
    control::{Control, Key},
    display::{Display, Line},
    error::StateTransitionError,
    transition::Transition,
    variable_mapping::VariableMapper, workflow::CommandRunner,
};

pub(crate) mod builder;

#[derive(Clone)]
pub(crate) struct State<R: CommandRunner, M: VariableMapper> {
    display_name: String,
    command_output_to_display: M,
    transitions: HashMap<Key, Transition<R, M>>,
    command_runner: R,
    display: Display,
}

impl<R: CommandRunner, M: VariableMapper> State<R, M> {
    pub(crate) fn new(
        display_name: &str,
        command_output_to_display: M,
        transitions: Vec<Transition<R, M>>,
        command_runner: R,
    ) -> Self {
        let transition_mapping = transitions
            .into_iter()
            .map(|t: Transition<R, M>| (t.get_activation_control().get_key(), t))
            .collect::<HashMap<Key, Transition<R, M>>>();

        Self {
            display_name: String::from(display_name),
            command_output_to_display,
            transitions: transition_mapping,
            command_runner,
            display: Display::default(),
        }
    }

    pub(crate) fn transition(
        &mut self,
        display_selection: Option<Line>,
        key: &Key,
    ) -> Result<Rc<RefCell<State<R, M>>>, StateTransitionError> {
        if let Some(transition) = self.transitions.get(key) {
            let transition_command = transition.get_transition_command(display_selection);
            if let Ok(command_to_execute) = transition_command {
                let cli_result = self.command_runner.run_command(&command_to_execute);
                if let Ok(cli_output) = cli_result {
                    let next_state = transition.get_next_state();
                    _ = next_state.try_borrow_mut() //TODO: fix this hack
                        .and_then(|mut next_state| {
                            next_state.display(&cli_output);
                            Ok(())
                        })
                        .or_else(|_| { // Next state is this state
                            self.display(&cli_output);
                            Ok::<_,BorrowMutError>(())
                        });
                    Ok(next_state.clone())
                } else {
                    Err(StateTransitionError::CliCommandExecutionFailed(
                        command_to_execute.clone(),
                    ))
                }
            } else {
                let transition_error = transition_command.unwrap_err();
                Err(StateTransitionError::SelectionToCommandMappingFailed(
                    transition_error,
                ))
            }
        } else {
            Err(StateTransitionError::ControlNotFound(*key))
        }
    }

    pub(crate) fn get_controls(&self) -> Vec<&Control> {
        self.transitions
            .values()
            .map(|transition| transition.get_activation_control())
            .collect()
    }

    pub(crate) fn add_transition(&mut self, key: Key, transition: Transition<R, M>) {
        self.transitions.insert(key, transition);
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.display_name
    }

    pub(crate) fn get_display(&self) -> &Display {
        &self.display
    }

    fn parse_display(&self, command_output: &str) -> Display {
        let mut lines = Vec::new();
        for line_result in self.command_output_to_display.map(command_output) {
            match line_result {
                Ok(line) => lines.push(Line(line)),
                Err(_e) => (),
            }
        }

        Display {
            lines,
            ..Default::default()
        }
    }

    fn display(&mut self, display: &str) {
        self.display = self.parse_display(display);
    }
}

/*
#[cfg(test)]
mod state_tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{model::{
            control::Key, error::StateTransitionError, state::builder::StateBuilder, transition::builder::TransitionBuilder, variable_mapping::{MockVariableMapper, VariableMappingError}, Control
        }, workflow::MockCommandRunner};

    #[test]
    fn transition_with_unexisting_controlkey_returns_error() {
        // Arrange
        let state_under_test = StateBuilder::<
            MockCommandRunner,
            MockVariableMapper,
        >::new()
        .with_display_name("test_state".to_string())
        .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
        .build();

        let non_existing_control = Control::new("non_existing_control", Key::Char('c'));

        // Act
        let result = state_under_test.transition("test_selection", &non_existing_control.get_key());

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            StateTransitionError::ControlNotFound(non_existing_control.get_key())
        )
    }

    #[test]
    fn transition_with_failing_command_returns_error() {
        // Arrange
        let fake_control = Control::new("non_existing_control", Key::Char('c'));
        let fake_target_state = StateBuilder::new()
            .with_display_name("TARGET".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .build();

        let fake_transition = TransitionBuilder::new()
            .with_control(fake_control.clone())
            .with_next_state(Rc::new(fake_target_state))
            .with_selected_display_to_command(get_mock_variable_mapper(Ok("TEST".to_string())))
            .build();

        let state_under_test = StateBuilder::new()
            .with_display_name("test_state".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .add_transition(fake_transition)
            .build();

        // Act
        let display_selection = "test_selection";
        let result = state_under_test.transition(&display_selection, &fake_control.get_key());

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            StateTransitionError::CliCommandExecutionFailed("TEST".to_string())
        )
    }

    #[test]
    fn transition_with_failing_selection_to_command_mapping_returns_error() {
        // Arrange
        let mock_context_ref = Rc::new(RefCell::new(mock_context));
        let fake_control = Control::new("non_existing_control", Key::Char('c'));
        let fake_target_state = StateBuilder::new()
            .with_display_name("TARGET".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .with_context(Rc::clone(&mock_context_ref))
            .build();

        let fake_transition = TransitionBuilder::new()
            .with_control(fake_control.clone())
            .with_next_state(Rc::new(fake_target_state))
            .with_selected_display_to_command(get_mock_variable_mapper(Err(VariableMappingError)))
            .build();

        let state_under_test = StateBuilder::new()
            .with_display_name("test_state".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .with_context(mock_context_ref)
            .add_transition(fake_transition)
            .build();

        // Act
        let display_selection = "test_selection";
        let result = state_under_test.transition(&display_selection, &fake_control.get_key());

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            StateTransitionError::SelectionToCommandMappingFailed(VariableMappingError)
        )
    }

    // TODO: Add successful test :-)

    fn get_mock_variable_mapper(
        single_map_result: Result<String, VariableMappingError>,
    ) -> MockVariableMapper {
        let mut mock_variable_mapper = MockVariableMapper::new();
        let single_map_result_recurse = single_map_result.clone(); // TODO find a more elegant solution to this and to cloning in every capture
        mock_variable_mapper
            .expect_map()
            .returning(move |_| Box::new(vec![single_map_result.clone()].into_iter()));
        mock_variable_mapper
            .expect_clone()
            .returning(move || get_mock_variable_mapper(single_map_result_recurse.clone()));
        mock_variable_mapper
    }
}
*/
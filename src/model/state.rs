use std::{cell::RefCell, collections::HashMap, rc::Rc};
use mockall::automock;

use super::{control::Control, display::{Display, Line}, error::StateTransitionError, transition::Transition, variable_mapping::{RegexVariableMapper, VariableMapper}};

#[derive(Clone)]
pub(crate) struct State<C: StateContext<M>, M: VariableMapper> {
    display_name: String,
    command_output_to_display: M,
    transitions: HashMap<String, Transition<C, M>>,
    context: Rc<RefCell<C>>,
}

impl<C: StateContext<M>, M: VariableMapper> State<C, M> {
    pub(crate) fn new(
        display_name: &str, 
        command_output_to_display: M,
        context: Rc<RefCell<C>>,
        transitions: Vec<Transition<C, M>>) -> Self {
        let transition_mapping = transitions
            .into_iter()
            .map(|t: Transition<C, M>| (String::from(t.get_activation_control().get_key()), t))
            .collect::<HashMap<String, Transition<C, M>>>();

        Self {
            display_name: String::from(display_name),
            command_output_to_display,
            transitions: transition_mapping,
            context,
        }
    }

    pub(crate) fn add_transition(&mut self, transition: Transition<C, M>) {
        self.transitions.insert(String::from(transition.get_activation_control().get_key()), transition);
    }

    pub(crate) fn transition(&self, display_selection: &str, control: &Control) -> Result<(), StateTransitionError> { // TODO Change this to take a key, not a whole control, also: wrap key into a value-object
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
            Err(StateTransitionError::ControlNotFound(control.clone()))
        }
    }
    
    pub(crate) fn get_controls(&self) -> Vec<Control> {
        self.transitions
            .values()
            .map(|transition| {
                transition.get_activation_control().clone()
            })
            .collect()
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

#[automock]
pub(crate) trait StateContext<M: VariableMapper> : Sized {
    fn update(&mut self, state: Rc<State<Self, M>>, display: Display);
    fn run_command(&self, command: &str) -> Result<String, ()>;
}

#[cfg(test)]
mod state_tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{builders::{StateBuilder, TransitionBuilder}, model::{error::StateTransitionError, state::MockStateContext, variable_mapping::{MockVariableMapper, VariableMappingError}, Control}};

    #[test]
    fn transition_with_unexisting_controlkey_returns_error() {
        // Arrange
        let state_under_test = StateBuilder::<MockStateContext<MockVariableMapper>, MockVariableMapper>::new()
            .with_display_name("test_state".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .with_context(Rc::new(RefCell::new(MockStateContext::new())))
            .build();

        let non_existing_control = Control::new("non_existing_control", "c");

        // Act
        let result = state_under_test.transition("test_selection", &non_existing_control);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), StateTransitionError::ControlNotFound(non_existing_control))
    }

    #[test]
    fn transition_with_failing_command_returns_error() {
        // Arrange
        let mut mock_context = MockStateContext::new();
        mock_context
            .expect_run_command()
            .return_const(Err(()));

        let mock_context_ref = Rc::new(RefCell::new(mock_context));
        let fake_control = Control::new("non_existing_control", "c");
        let fake_target_state = StateBuilder::new()
            .with_display_name("TARGET".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .with_context(Rc::clone(&mock_context_ref))
            .build();

        let fake_transition = TransitionBuilder::new()
            .with_control(fake_control.clone())
            .with_next_state(Rc::new(fake_target_state))
            .with_selected_display_to_command(get_mock_variable_mapper(Ok("TEST".to_string())))
            .build();

        let state_under_test = StateBuilder::new()
            .with_display_name("test_state".to_string())
            .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
            .with_context(mock_context_ref)
            .add_transition(fake_transition)
            .build();

        // Act
        let display_selection = "test_selection";
        let result = state_under_test.transition(&display_selection, &fake_control);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), StateTransitionError::CliCommandExecutionFailed("TEST".to_string()))
    }

    #[test]
    fn transition_with_failing_selection_to_command_mapping_returns_error() {
        // Arrange
        let mut mock_context = MockStateContext::new();
        mock_context
            .expect_run_command()
            .return_const(Ok("GREAT SUCCESS".to_string()));

        let mock_context_ref = Rc::new(RefCell::new(mock_context));
        let fake_control = Control::new("non_existing_control", "c");
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
        let result = state_under_test.transition(&display_selection, &fake_control);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), StateTransitionError::SelectionToCommandMappingFailed(VariableMappingError))
    }

    // TODO: Add successful test :-)

    fn get_mock_variable_mapper(single_map_result: Result<String, VariableMappingError>) -> MockVariableMapper {
        let mut mock_variable_mapper = MockVariableMapper::new();
        let single_map_result_recurse = single_map_result.clone(); // TODO find a more elegant solution to this and to cloning in every capture
        mock_variable_mapper.expect_map().returning(move |_| Box::new(vec![single_map_result.clone()].into_iter()));
        mock_variable_mapper.expect_clone().returning(move || get_mock_variable_mapper(single_map_result_recurse.clone()));
        mock_variable_mapper
    }
}
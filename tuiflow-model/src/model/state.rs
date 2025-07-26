mod workflow_state;
mod state;

pub use workflow_state::*;
pub use state::*;

// #[cfg(test)]
// mod state_tests {
//     use crate::model::state::State;
//     use crate::model::transition::{DisplayToCommandMappingError, Transition};
//     use crate::model::{
//         control::Key,
//         error::StateTransitionError,
//         state::builder::StateBuilder,
//         transition::builder::TransitionBuilder,
//         variable_mapping::{MockVariableMapper, VariableMappingError},
//         Control,
//     };
//     use std::{cell::RefCell, rc::Rc};
//     pub use tuiflow_model_contracts::command_runner::CommandRunner;
//     use tuiflow_model_contracts::command_runner::{CommandRunnerError, MockCommandRunner};
//
//     #[test]
//     fn transition_with_unexisting_controlkey_returns_error() {
//         // Arrange
//         let mut state_under_test = get_testee(get_mock_command_runner(Ok("TEST".to_string())));
//         let non_existing_control = Control::new("non_existing_control", Key::Char('c'));
//
//         // Act
//         let result = state_under_test.transition(Some("test"), &non_existing_control.get_key());
//
//         // Assert
//         assert!(result.is_err());
//         assert_eq!(
//             result.err().unwrap(),
//             StateTransitionError::ControlNotFound(non_existing_control.get_key())
//         )
//     }
//
//     #[test]
//     fn transition_with_failing_command_returns_error() {
//         // Arrange
//         let fake_control = Control::new("non_existing_control", Key::Char('c'));
//         let fake_target_state = StateBuilder::new()
//             .with_display_name("TARGET".to_string())
//             .with_command_runner(get_mock_command_runner(Ok("TEST".to_string())))
//             .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
//             .build();
//         let mut state_under_test = get_testee(get_mock_command_runner(Err(CommandRunnerError {
//             command: "TEST".to_string(),
//         })));
//         let fake_variable_mapper = get_mock_variable_mapper(Ok("TEST".to_string()));
//         let fake_transition =
//             get_mock_transition(&fake_control, fake_target_state, fake_variable_mapper);
//
//         state_under_test.add_transition(fake_control.get_key(), fake_transition);
//
//         // Act
//         let result = state_under_test.transition(Some("test"), &fake_control.get_key());
//
//         // Assert
//         assert!(result.is_err());
//         assert_eq!(
//             result.err().unwrap(),
//             StateTransitionError::CommandExecutionFailed("TEST".to_string())
//         )
//     }
//
//     #[test]
//     fn transition_with_failing_selection_to_command_mapping_returns_error() {
//         // Arrange
//         let fake_control = Control::new("control", Key::Char('c'));
//         let fake_target_state = get_mock_state();
//
//         let fake_variable_mapper = get_mock_variable_mapper(Err(VariableMappingError));
//         let fake_transition =
//             get_mock_transition(&fake_control, fake_target_state, fake_variable_mapper);
//         let mut state_under_test = get_testee(get_mock_command_runner(Ok("TEST".to_string())));
//
//         state_under_test.add_transition(fake_control.get_key(), fake_transition);
//
//         // Act
//         let result = state_under_test.transition(Some("test"), &fake_control.get_key());
//
//         // Assert
//         assert!(result.is_err());
//         assert_eq!(
//             result.err().unwrap(),
//             StateTransitionError::SelectionToCommandMappingFailed(
//                 DisplayToCommandMappingError::VariableMappingError(VariableMappingError)
//             )
//         )
//     }
//
//     // TODO: Add successful test :-)
//     #[test]
//     fn transition_with_successful_command_execution_returns_next_state() {
//         // Arrange
//         let fake_control = Control::new("test_control", Key::Char('c'));
//         let fake_target_state = get_mock_state();
//         let fake_variable_mapper = get_mock_variable_mapper(Ok("TEST".to_string()));
//
//         let fake_transition =
//             get_mock_transition(&fake_control, fake_target_state, fake_variable_mapper);
//
//         let mut state_under_test = get_testee(get_mock_command_runner(Ok("TEST".to_string())));
//         state_under_test.add_transition(fake_control.get_key(), fake_transition);
//
//         // Act
//         let result = state_under_test.transition(Some("test"), &fake_control.get_key());
//
//         // Assert
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap().borrow().get_name(), "TARGET");
//     }
//
//     fn get_mock_transition(
//         fake_control: &Control,
//         fake_target_state: State<MockCommandRunner, MockVariableMapper>,
//         variable_mapper: MockVariableMapper,
//     ) -> Transition<MockCommandRunner, MockVariableMapper> {
//         TransitionBuilder::new()
//             .with_control(fake_control.clone())
//             .with_next_state(Rc::new(RefCell::new(fake_target_state)))
//             .with_selected_display_to_command(variable_mapper)
//             .build()
//     }
//
//     fn get_mock_state() -> State<MockCommandRunner, MockVariableMapper> {
//         StateBuilder::new()
//             .with_display_name("TARGET".to_string())
//             .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
//             .with_command_runner(get_mock_command_runner(Ok("TEST".to_string())))
//             .build()
//     }
//
//     fn get_mock_variable_mapper(
//         single_map_result: Result<String, VariableMappingError>,
//     ) -> MockVariableMapper {
//         let mut mock_variable_mapper = MockVariableMapper::new();
//         let single_map_result_recurse = single_map_result.clone();
//         mock_variable_mapper
//             .expect_map()
//             .returning(move |_| Box::new(vec![single_map_result.clone()].into_iter()));
//         mock_variable_mapper
//             .expect_clone()
//             .returning(move || get_mock_variable_mapper(single_map_result_recurse.clone()));
//         mock_variable_mapper
//     }
//
//     fn get_mock_command_runner(result: Result<String, CommandRunnerError>) -> MockCommandRunner {
//         let mut mock_command_runner = MockCommandRunner::default();
//         let result_recurse = result.clone();
//         mock_command_runner
//             .expect_run_command()
//             .returning(move |_| result.clone());
//         mock_command_runner
//             .expect_clone()
//             .returning(move || get_mock_command_runner(result_recurse.clone()));
//         mock_command_runner
//     }
//
//     fn get_testee(
//         command_runner: MockCommandRunner,
//     ) -> State<MockCommandRunner, MockVariableMapper> {
//         StateBuilder::new()
//             .with_display_name("test_state".to_string())
//             .with_command_output_to_display_mapper(get_mock_variable_mapper(Ok("TEST".to_string())))
//             .with_command_runner(command_runner)
//             .build()
//     }
// }

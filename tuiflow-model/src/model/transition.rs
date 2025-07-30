use crate::model::variable::VariableSet;
use crate::model::variable_mapping::VariableExtractor;
use crate::state::State;
use crate::state::Transit;
use crate::state::WorkflowState;
use crate::variable_mapping::VariableInjector;
use std::{cell::RefCell, rc::Rc};
use tuiflow_model_contracts::command_runner::CommandRunner;
use tuiflow_model_contracts::control::Control;
use tuiflow_model_contracts::error::StateTransitionError;

pub struct Transition<R: CommandRunner, M: VariableExtractor> {
    control: Control,
    next_state: Rc<RefCell<WorkflowState<Self>>>, //TODO: Check and break cycles
    variable_set_command_filler: VariableInjector, // regex extraction from selection
    cli_output_variable_extractor: M,
    command_runner: R,
}

impl<R: CommandRunner, M: VariableExtractor> Transit for Transition<R, M> {
    fn run(&self, variables: &VariableSet) -> Result<State<Self>, StateTransitionError> {
        let transition_command = self
            .get_transition_command(variables);

        self.run_command(&transition_command)
    }

    fn get_activation_control(&self) -> &Control {
        &self.control
    }
}

impl<R: CommandRunner, M: VariableExtractor> Transition<R, M> {
    pub fn new(
        control: Control,
        next_state: Rc<RefCell<WorkflowState<Self>>>,
        variable_set_command_filler: VariableInjector,
        command_runner: R,
        cli_output_variable_extractor: M,
    ) -> Self {
        Self {
            control,
            next_state,
            variable_set_command_filler,
            command_runner,
            cli_output_variable_extractor,
        }
    }
    fn get_transition_command(
        &self,
        variables: &VariableSet,
    ) -> R::Command {
        self.variable_set_command_filler
            .inject(variables)
            .into()
    }

    fn run_command(
        &self,
        command_to_execute: &<R as CommandRunner>::Command,
    ) -> Result<State<Self>, StateTransitionError> {
        let cli_result = self
            .command_runner
            .run_command(&command_to_execute)
            .map_err(|e| StateTransitionError::CommandExecutionError(e))?;

        let variables = self.cli_output_variable_extractor.extract(&cli_result);
        Ok(State::new(Rc::clone(&self.next_state), variables))
    }
}

impl<R: CommandRunner, M: VariableExtractor> Clone for Transition<R, M> {
    fn clone(&self) -> Self {
        Self {
            control: self.control.clone(),
            next_state: Rc::clone(&self.next_state),
            variable_set_command_filler: self.variable_set_command_filler.clone(),
            command_runner: self.command_runner.clone(),
            cli_output_variable_extractor: self.cli_output_variable_extractor.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use tuiflow_model_contracts::command_runner::MockCommand;
    use tuiflow_model_contracts::command_runner::MockCommandRunner;
    use tuiflow_model_contracts::control::{Control, Key};
    use crate::model::variable::VariableSet;
    use crate::state::{Transit, WorkflowState};
    use crate::transition::Transition;
    use crate::variable_mapping::{MockVariableExtractor, VariableInjector};

    #[test]
    fn run_runs_command_returned_by_variable_filler() {
        let expected_command = MockCommand::from("rm all_and_everything".to_string());
        let variable_injector = VariableInjector::new(expected_command.command.clone());
        let workflow_state = Rc::new(RefCell::new(WorkflowState::new("state", variable_injector.clone(), vec![])));
        let mut command_runner = MockCommandRunner::default();
        command_runner
            .expect_run_command()
            .once()
            .withf(move |cmd| cmd.command == expected_command.command)
            .returning(|_| Ok("and there was nothing".to_string()));
        let mut variable_extractor = MockVariableExtractor::new();
        variable_extractor
            .expect_extract()
            .once()
            .returning(|_| vec![]);
        let transition = Transition::new(Control::new("ctrl", Key::Esc), workflow_state.clone(), variable_injector, command_runner, variable_extractor);
        let variable_set = VariableSet::empty();
        _ = transition.run(&variable_set);
    }
    
    #[test]
    fn run_extracts_variables_from_command_result() {
        let expected_command = MockCommand::from("rm all_and_everything".to_string());
        let variable_injector = VariableInjector::new(expected_command.command.clone());
        let workflow_state = Rc::new(RefCell::new(WorkflowState::new("state", variable_injector.clone(), vec![])));
        let cli_output = "and there was nothing";
        let mut command_runner = MockCommandRunner::default();
        command_runner
            .expect_run_command()
            .once()
            .withf(move |cmd| cmd.command == expected_command.command)
            .returning(|_| Ok(cli_output.to_string()));
        let mut variable_extractor = MockVariableExtractor::new();
        variable_extractor
            .expect_extract()
            .once()
            .withf(move |input| input == cli_output)
            .returning(|_| vec![]);
        let transition = Transition::new(Control::new("ctrl", Key::Esc), workflow_state.clone(), variable_injector, command_runner, variable_extractor);
        let variable_set = VariableSet::empty();
        _ = transition.run(&variable_set);

    }
}

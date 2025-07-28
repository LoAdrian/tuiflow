use crate::model::variable::VariableSet;
use crate::model::variable_mapping::VariableExtractor;
use crate::state;
use crate::state::State;
use crate::state::WorkflowState;
use crate::variable_mapping::VariableInjector;
use std::{cell::RefCell, rc::Rc};
use tuiflow_model_contracts::command_runner::CommandRunner;
use tuiflow_model_contracts::control::Control;
use tuiflow_model_contracts::error::{StateTransitionError, VariableMappingError};

pub struct Transition<R: CommandRunner, M: VariableExtractor> {
    control: Control,
    next_state: Rc<RefCell<WorkflowState<Self>>>, //TODO: Check and break cycles
    variable_set_command_filler: VariableInjector, // regex extraction from selection
    cli_output_variable_extractor: M,
    command_runner: R,
}

impl<R: CommandRunner, M: VariableExtractor> state::Transition for Transition<R, M> {


    fn run(&self, variables: &VariableSet) -> Result<State<Self>, StateTransitionError> {
        let transition_command = self
            .get_transition_command(variables)
            .map_err(|e| StateTransitionError::VariableMappingError(e))?;

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
    ) -> Result<R::Command, VariableMappingError> {
        self.variable_set_command_filler
            .inject(variables)
            .map(|command| command.into())
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

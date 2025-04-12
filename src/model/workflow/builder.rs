use std::rc::Rc;

use crate::model::{
    state::State,
    variable_mapping::VariableMapper,
    workflow::{CommandRunner, Workflow},
};

use crate::model::state::builder::StateBuilder;

#[derive(Clone)]
pub struct WorkflowBuilder<R: CommandRunner, M: VariableMapper> {
    command_runner: Option<R>,
    initial_state: Option<Rc<State<R, M>>>,
    initial_command: Option<String>,
    initial_command_output_to_display: Option<M>,
}

impl<C: CommandRunner, M: VariableMapper> Default for WorkflowBuilder<C, M> {
    fn default() -> Self {
        Self {
            command_runner: Default::default(),
            initial_state: Default::default(),
            initial_command: Default::default(),
            initial_command_output_to_display: Default::default(),
        }
    }
}
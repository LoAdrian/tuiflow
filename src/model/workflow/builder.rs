use std::rc::Rc;

use crate::model::{
    state::State,
    variable_mapping::VariableMapper,
    workflow::{CommandRunner, Workflow},
};

use crate::model::state::builder::StateBuilder;

#[derive(Clone)]
pub struct WorkflowBuilder<C: CommandRunner, M: VariableMapper> {
    command_runner: Option<C>,
    initial_state: Option<Rc<State<Workflow<C, M>, M>>>,
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

impl<C: CommandRunner, M: VariableMapper> WorkflowBuilder<C, M> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_command_runner(&mut self, command_runner: C) -> &mut Self {
        self.command_runner = Some(command_runner);
        self
    }

    pub fn with_initial_state(
        &mut self,
        initial_state: &Rc<State<Workflow<C, M>, M>>,
    ) -> &mut Self {
        self.initial_state = Some(Rc::clone(initial_state));
        self
    }

    pub fn with_initial_command(&mut self, initial_command: &str) -> &mut Self {
        self.initial_command = Some(initial_command.to_string());
        self
    }

    pub fn with_initial_command_output_to_display(
        &mut self,
        initial_command_output_to_display: M,
    ) -> &mut Self {
        self.initial_command_output_to_display = Some(initial_command_output_to_display);
        self
    }

    pub fn build_initial_state(
        &mut self,
        f: impl FnOnce(StateBuilder<Workflow<C, M>, M>) -> State<Workflow<C, M>, M>,
    ) -> &mut Self {
        let state = f(StateBuilder::<Workflow<C, M>, M>::default());
        self.initial_state = Some(Rc::new(state));
        self
    }

    pub fn build(&self) -> Workflow<C, M> {
        let mut workflow = Workflow::new(
            self.command_runner
                .clone()
                .expect("Command runner is required to build Workflow"),
        );
        if let (
            Some(initial_state),
            Some(initial_command),
            Some(initial_command_output_to_display),
        ) = (
            &self.initial_state,
            &self.initial_command,
            self.initial_command_output_to_display.as_ref(),
        ) {
            workflow.init(
                Rc::clone(initial_state),
                initial_command,
                initial_command_output_to_display.clone(),
            );
        }
        workflow
    }
}

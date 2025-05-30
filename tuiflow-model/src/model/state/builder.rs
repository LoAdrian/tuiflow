use crate::command_runner::CommandRunner;
use crate::state::State;
use crate::transition::Transition;
use crate::variable_mapping::VariableMapper;

#[derive(Clone)]
pub struct StateBuilder<R: CommandRunner + Clone, M: VariableMapper> {
    display_name: Option<String>,
    command_output_to_display_mapper: Option<M>,
    command_runner: Option<R>,
    transitions: Vec<Transition<R, M>>,
}

impl<R: CommandRunner + Clone, M: VariableMapper> Default for StateBuilder<R, M> {
    fn default() -> Self {
        Self { 
            display_name: Default::default(),
            command_output_to_display_mapper: Default::default(),
            command_runner: Default::default(),
            transitions: Default::default(),
        }
    }
}

impl<R: CommandRunner + Clone, M: VariableMapper> StateBuilder<R, M> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_display_name(&mut self, display_name: String) -> &mut Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn with_command_output_to_display_mapper(
        &mut self,
        command_output_to_display_mapper: M,
    ) -> &mut Self {
        self.command_output_to_display_mapper = Some(command_output_to_display_mapper);
        self
    }

    pub fn with_command_runner(&mut self, command_runner: R) -> &mut Self {
        self.command_runner = Some(command_runner);
        self
    }

    pub fn build(&self) -> State<R, M> {
        // Consume self. Force clone(). Implies that clone is called on underlying types.
        State::new(
            self.display_name
                .as_ref()
                .expect("Display name is required"),
            self.command_output_to_display_mapper
                .as_ref()
                .expect("Command output to display mapper is required")
                .clone(),
            self.transitions.clone(),
            self.command_runner.as_ref().expect("CommandRunner is required").clone(),
        )
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use crate::model::control::Key;
use crate::model::transition::builder::TransitionBuilder;
use crate::model::{state::State, variable_mapping::VariableMapper, workflow::{CommandRunner, Workflow}, Control};

#[derive(Clone)]
pub struct WorkflowBuilder<'a, R: CommandRunner + Clone, M: VariableMapper> {
    initial_state: Option<&'a Rc<RefCell<State<R, M>>>>,
    initial_display_to_command_mapper: Option<M>,
    command_runner: Option<R>,
    app_title: Option<&'a str>,
}

impl<'a, C: CommandRunner + Clone, M: VariableMapper> WorkflowBuilder<'a, C, M> {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn with_initial_state(&mut self, initial_state: &'a Rc<RefCell<State<C, M>>>) -> &mut Self {
        self.initial_state = Some(initial_state);
        self
    }
    
    pub fn with_initial_display_to_command_mapper(&mut self, mapper: M) -> &mut Self {
        self.initial_display_to_command_mapper = Some(mapper);
        self
    }

    pub fn with_command_runner(&mut self, command_runner: C) -> &mut Self {
        self.command_runner = Some(command_runner);
        self
    }
    
    pub fn with_app_title(&mut self, app_title: &'a str) -> &mut Self {
        self.app_title = Some(app_title);
        self
    }

    pub fn build(&self) -> Workflow<C, M> {

        let initial_transition = TransitionBuilder::new()
            .with_control(Control::new("", Key::Backspace)) //unimportant
            .with_selected_display_to_command(self.initial_display_to_command_mapper.clone().unwrap().clone())
            .with_next_state(self.initial_state.clone().unwrap().clone())
            .build();
        let initializer_state = State::new("INIT", M::identity(), vec![initial_transition], self.command_runner.clone().unwrap());

        Workflow::<C, M>::new(initializer_state, self.app_title.unwrap().to_string())
    }
}


impl<'a, C: CommandRunner + Clone, M: VariableMapper> Default for WorkflowBuilder<'a, C, M> {
    fn default() -> Self {
        Self {
            initial_display_to_command_mapper: Default::default(),
            initial_state: Default::default(),
            command_runner: Default::default(),
            app_title: Default::default(),
        }
    }
}
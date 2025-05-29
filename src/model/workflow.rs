use mockall::mock;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{
    cell::{Ref, RefCell},
    process::Command,
    rc::Rc,
};

use super::{control::{Control, Key}, display, error::StateTransitionError, state::State, variable_mapping::VariableMapper, Line, TerminalFlow};

pub(crate) mod builder;

pub(crate) struct Workflow<R: CommandRunner, M: VariableMapper> {
    current_state: Rc<RefCell<State<R, M>>>,
    app_title: String,
}

impl<R: CommandRunner, M: VariableMapper> Workflow<R, M> {
    pub fn new(initializer_state: State<R, M>, app_title: String) -> Result<Self, InitialTransitionError> {
        let mut initializer_state_mut = initializer_state;
        let init_control = initializer_state_mut
            .get_controls()
            .pop()
            .expect("Initializer state must contain at least one control. Please report this issue on github.");
        let current_state = initializer_state_mut
            .transition(None, &init_control.get_key()).map_err(|e| InitialTransitionError(e))?;
        Ok(Self {
            current_state,
            app_title,
        })
    }
}

#[derive(Debug)]
pub(crate) struct InitialTransitionError(StateTransitionError);

impl Display for InitialTransitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initial transition error: {}", self.0)
    }
}

impl Error for InitialTransitionError {}

// TODO: Probably put this and impl to somewhere else
impl<R: CommandRunner, M: VariableMapper> TerminalFlow for Workflow<R, M> {
    fn run_control(
        &mut self,
        display_selection_index: usize,
        key: &Key,
    ) -> Result<(), StateTransitionError> {
        let transition_result: Result<Rc<RefCell<State<R, M>>>, StateTransitionError>;
        {
            let selected_line: Option<Line>;

            {
                //Scope RefCell borrow
                // TODO move all this to state
                let display = self.get_display();
                selected_line = display
                    .lines
                    .get(display_selection_index)
                    .map(|l| l.clone()); // double borrow if we don't do this // TODO: eliminate this smell
            }

            transition_result = self
                .current_state
                .borrow_mut()
                .transition(selected_line, key);
        }

        match transition_result {
            Ok(next_state) => {
                self.current_state = next_state;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get_display(&self) -> Ref<display::Display> {
        Ref::map(self.current_state.borrow(), |x| x.get_display())
    }

    fn get_state_title(&self) -> Ref<str> {
        Ref::map(self.current_state.borrow(), |s| s.get_name())
    }

    fn get_app_title(&self) -> &str {
        &self.app_title
    }

    fn get_state_controls<'a>(&'a self) -> Vec<Control> {
        let current_state: Ref<'a, State<R, M>> = self.current_state.borrow();
        current_state
            .get_controls()
            .into_iter()
            .map(|c| c.clone())
            .collect()
    }
}

pub trait CommandRunner: Clone {
    fn run_command(&self, command: &str) -> Result<String, CommandRunnerError>;
}

mock! {
    pub(crate) CommandRunner {}
    
    impl Clone for CommandRunner {
        fn clone(&self) -> Self;
    }
    
    impl CommandRunner for CommandRunner {
        fn run_command(&self, command: &str) -> Result<String, CommandRunnerError>;
    }
}

#[derive(Debug, Clone)]
pub struct CommandRunnerError {
    pub command: String,
}

impl Display for CommandRunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Failed to run command '{}'", self.command)
    }
}

impl Error for CommandRunnerError {}

//move this out of the model
#[derive(Clone)]
pub struct ShCommandRunner;

impl CommandRunner for ShCommandRunner {
    fn run_command(&self, command: &str) -> Result<String, CommandRunnerError> {
        let cli_result = Command::new("sh").arg("-c").arg(command).output();
        if let Ok(cli_output) = cli_result {
            if let Ok(cli_result) = String::from_utf8(cli_output.stdout) {
                return Ok(cli_result)
            }
        }

        Err(CommandRunnerError {
            command: format!("{} {}", "sh -c", command),
        })
    }
}

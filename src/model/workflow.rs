use std::{process::Command, rc::Rc};

use super::{display::Display, error::StateTransitionError, state::{State, StateContext}};

pub(crate) struct Workflow {
    current_display: Display,
    current_state: Rc<State<Self>>,
    line_delimiter: char,
}

impl Workflow {
    fn parse_display(raw_content: String, line_delimiter: char) -> Display {
        todo!();
    }
}

impl StateContext for Workflow {
    fn update(&mut self, state: Rc<State<Self>>, command: String) -> Result<(), StateTransitionError> {
        let cli_result = Command::new("sh") // TODO: Make configurable by workflow
            .arg("-c") // TODO: Make configurable by workflow
            .arg(command.clone())
            .output();

        if let Ok(cli_output) = cli_result{
            self.current_display = Self::parse_display(String::from_utf8(cli_output.stdout).unwrap(), self.line_delimiter);
            self.current_state = state;
            Ok(())
        } else {
            Err(StateTransitionError::CliCommandExecutionFailed(command))
        }
    }
}

// TODO: Probably put this and impl to somewhere else
pub trait Terminal {
    fn run_command(&mut self, display_selection: &str, command: char);
    fn get_display(&self) -> &Display;
}

impl Terminal for Workflow {
    fn run_command(&mut self, display_selection: &str, control: char) {
        if let Err(e) = self.current_state.transition(display_selection, control) {
            self.current_display.error = String::from(format!("{e}"));
        }
    }

    fn get_display(&self) -> &Display {
        &self.current_display
    }
}



trait CommandRunner {
    fn run(&self, command: &str) -> String;
}
use std::{process::Command, rc::Rc};

use ratatui::init;
use regex::Regex;

use super::{control::Control, display::Display, state::{State, StateContext}, transition::Transition, variable_mapping::VariableMapper, Terminal};

pub(crate) struct Workflow<R: CommandRunner> {
    current_display: Option<Display>,
    current_state: Option<Rc<State<Self>>>,
    command_runner: R
    
}

impl<R: CommandRunner> Workflow<R> {
    pub fn new(command_runner: R) -> Self {
        Self {
            current_display: None,
            current_state: None,
            command_runner
        }
    }
    
    pub fn init(&mut self, initial_state: Rc<State<Self>>, initial_command: &str, initial_command_to_display: VariableMapper) {

    }

    fn get_current_display(&self) -> &Display {
        self.current_display.as_ref().expect("Workflow is uninitialized. Call init first.")
    }

    fn get_current_state(&self) -> &Rc<State<Self>> {
        self.current_state.as_ref().expect("Workflow is uninitialized. Call init first.")
    }

    fn get_current_display_mut(&mut self) -> &mut Display {
        self.current_display.as_mut().expect("Workflow is uninitialized. Call init first.")
    }

    fn get_current_state_mut(&mut self) -> &mut Rc<State<Self>> {
        self.current_state.as_mut().expect("Workflow is uninitialized. Call init first.")
    }
}

impl<R: CommandRunner> StateContext for Workflow<R> {
    fn update(&mut self, state: Rc<State<Self>>, new_display: Display) {
        *self.get_current_display_mut() = new_display;
        *self.get_current_state_mut() = state;
    }

    fn run_command(&self, command: &str) -> Result<String, ()> { // Might be a good idea to extract this
        self.command_runner.run_command(command)
    }
}

// TODO: Probably put this and impl to somewhere else
impl<R: CommandRunner> Terminal for Workflow<R> {
    fn run_command(&mut self, display_selection: &str, control: Control) {
        if let Err(e) = self.get_current_state().transition(display_selection, control) {
            self.get_current_display_mut().errors.push(String::from(format!("{e}")));
        }
    }

    fn get_display(&self) -> &Display {
        self.get_current_display()
    }
}

pub trait CommandRunner {
    fn run_command(&self, command: &str) -> Result<String, ()>;
}

//move this out of the model
pub struct ShCommandRunner;

impl CommandRunner for ShCommandRunner { 
    fn run_command(&self, command: &str) -> Result<String, ()> {
        let cli_result = Command::new("sh") 
            .arg("-c") 
            .arg(command.clone())
            .output();
        if let Ok(cli_output) = cli_result {
            Ok(String::from_utf8(cli_output.stdout).unwrap())
        } else {
            Err(())
        }
    }
}
use std::{process::Command, rc::Rc};

use super::{
    control::Control,
    display::Display,
    state::{State, StateContext},
    variable_mapping::VariableMapper,
    Line, TerminalFlow,
};

pub(crate) mod builder;

pub(crate) struct Workflow<R: CommandRunner, M: VariableMapper> {
    current_display: Option<Display>,
    current_state: Option<Rc<State<Self, M>>>,
    command_runner: R,
}

impl<R: CommandRunner, M: VariableMapper> Workflow<R, M> {
    pub fn new(command_runner: R) -> Self {
        Self {
            current_display: None,
            current_state: None,
            command_runner,
        }
    }

    pub fn init(
        &mut self,
        initial_state: Rc<State<Self, M>>,
        initial_command: &str,
        initial_command_output_to_display: M,
    ) {
        self.current_state = Some(initial_state);
        let command_output_result = self.command_runner.run_command(initial_command);
        self.current_display = Some(Display {
            lines: vec![],
            errors: vec![],
        });

        // TODO: DRY!
        if let Ok(command_output) = command_output_result {
            initial_command_output_to_display
                .map(command_output.as_str())
                .for_each(|line_result| {
                    let current_display = self.current_display.as_mut().unwrap();
                    if let Ok(line) = line_result {
                        current_display.lines.push(Line(line));
                    } else if let Err(e) = line_result {
                        current_display.errors.push(format!("{e}"));
                    }
                });
        } else {
            self.current_display
                .as_mut()
                .unwrap()
                .errors
                .push(String::from(format!(
                    "Failed to execute initial command {initial_command}"
                )));
        }
    }

    fn get_current_display(&self) -> &Display {
        self.current_display
            .as_ref()
            .expect("Workflow is uninitialized. Call init first.")
    }

    fn get_current_state(&self) -> &Rc<State<Self, M>> {
        self.current_state
            .as_ref()
            .expect("Workflow is uninitialized. Call init first.")
    }

    fn get_current_display_mut(&mut self) -> &mut Display {
        self.current_display
            .as_mut()
            .expect("Workflow is uninitialized. Call init first.")
    }

    fn get_current_state_mut(&mut self) -> &mut Rc<State<Self, M>> {
        self.current_state
            .as_mut()
            .expect("Workflow is uninitialized. Call init first.")
    }
}

impl<R: CommandRunner, M: VariableMapper> StateContext<M> for Workflow<R, M> {
    fn update(&mut self, state: Rc<State<Self, M>>, new_display: Display) {
        *self.get_current_display_mut() = new_display;
        *self.get_current_state_mut() = state;
    }

    fn run_command(&self, command: &str) -> Result<String, ()> {
        // Might be a good idea to extract this
        self.command_runner.run_command(command)
    }
}

// TODO: Probably put this and impl to somewhere else
impl<R: CommandRunner, M: VariableMapper> TerminalFlow for Workflow<R, M> {
    fn run_control(&mut self, display_selection: &str, control: &Control) {
        if let Err(e) = self
            .get_current_state()
            .transition(display_selection, control)
        {
            self.get_current_display_mut()
                .errors
                .push(String::from(format!("{e}")));
        }
    }

    fn get_display(&self) -> &Display {
        self.get_current_display()
    }

    fn get_controls(&self) -> Vec<Control> {
        self.get_current_state().get_controls()
    }
}

pub trait CommandRunner: Clone {
    fn run_command(&self, command: &str) -> Result<String, ()>;
}

//move this out of the model
#[derive(Clone)]
pub struct ShCommandRunner;

impl CommandRunner for ShCommandRunner {
    fn run_command(&self, command: &str) -> Result<String, ()> {
        let cli_result = Command::new("sh").arg("-c").arg(command).output();
        if let Ok(cli_output) = cli_result {
            Ok(String::from_utf8(cli_output.stdout).unwrap())
        } else {
            Err(())
        }
    }
}

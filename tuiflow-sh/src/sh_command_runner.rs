use tuiflow_model_contracts::command_runner::{CommandRunner, CommandRunnerError};
use std::process::Command;
use crate::sh_command::ShCommand;

#[derive(Clone)]
pub struct ShCommandRunner;

impl CommandRunner for ShCommandRunner {
    type Command = ShCommand;

    fn run_command(&self, command: &<Self as CommandRunner>::Command) -> Result<String, CommandRunnerError> {
        let command_str: &str = command;
        let cli_result = Command::new("sh").arg("-c").arg(command_str).output();
        if let Ok(cli_output) = cli_result {
            if let Ok(cli_result) = String::from_utf8(cli_output.stdout) {
                return Ok(cli_result)
            }
        }

        Err(CommandRunnerError {
            command: format!("{} {}", "sh -c", command_str),
        })
    }

    fn new() -> Self {
        ShCommandRunner
    }
}

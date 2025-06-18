use tuiflow_model::command_runner::CommandRunnerError;
use tuiflow_model::command_runner::CommandRunner;
use std::process::Command;

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

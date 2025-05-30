use std::error::Error;
use std::fmt::{Display, Formatter};
use mockall::mock;

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

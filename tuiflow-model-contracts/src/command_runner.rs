use mockall::mock;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

pub trait CommandRunner: Clone
where
{
    type Command: Deref<Target = str> + From<String>;
    fn run_command(&self, command: &Self::Command) -> Result<String, CommandRunnerError>;
    fn new() -> Self;
}

pub struct MockCommand {
    pub command: String,
}
impl<'a> From<String> for MockCommand {
    fn from(value: String) -> Self {
        Self {
            command: value.to_string(),
        }
    }
}

impl Deref for MockCommand {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.command.as_str()
    }
}

mock! {
    pub CommandRunner {}

    impl Clone for CommandRunner {
        fn clone(&self) -> Self;
    }

    impl CommandRunner for CommandRunner {
        type Command=MockCommand;
        fn run_command<'a>(&self, command: &<MockCommandRunner as CommandRunner>::Command) -> Result<String, CommandRunnerError>;
        fn new() -> Self;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandRunnerError {
    pub command: String,
}

impl Display for CommandRunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Failed to run command '{}'", self.command)
    }
}

impl Error for CommandRunnerError {}

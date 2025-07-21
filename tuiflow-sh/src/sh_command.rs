use std::ops::Deref;

pub struct ShCommand {
    command: String,
}

impl Deref for ShCommand {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl From<String> for ShCommand {
    fn from(value: String) -> Self {
        Self {
            command: value,
        }
    }
}
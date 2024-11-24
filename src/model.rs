pub(crate) mod variable_mapping;
pub(crate) mod state;
pub(crate) mod transition;
pub(crate) mod workflow;
pub(crate) mod error;
pub(crate) mod control;
pub(crate) mod display;
pub use control::Control;
pub use display::{Display, Line};

pub trait Terminal {
    fn run_command(&mut self, display_selection: &str, control: Control);
    fn get_display(&self) -> &Display;
}


pub(crate) mod control;
pub(crate) mod display;
pub(crate) mod error;
pub(crate) mod state;
pub(crate) mod transition;
pub(crate) mod variable_mapping;
pub(crate) mod workflow;
pub(crate) mod command_runner;

use std::cell::Ref;

pub use control::Control;
use control::Key;
pub use display::Display;
use error::StateTransitionError;

pub trait TerminalFlow {
    fn run_control(&mut self, display_selection_index: usize, key: &Key) -> Result<(), StateTransitionError>;
    fn get_display(&self) -> Ref<Display>;
    fn get_state_title(&self) -> Ref<'_, str>;
    fn get_app_title(&self) -> &str;
    fn get_state_controls(&self) -> Vec<Control>;
}

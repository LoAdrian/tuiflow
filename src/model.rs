pub(crate) mod control;
pub(crate) mod display;
pub(crate) mod error;
pub(crate) mod state;
pub(crate) mod transition;
pub(crate) mod variable_mapping;
pub(crate) mod workflow;
use std::cell::Ref;

pub use control::Control;
use control::Key;
pub use display::{Display, Line};
use error::StateTransitionError;

pub trait TerminalFlow {
    fn run_control(&mut self, display_selection_index: usize, key: &Key) -> Result<(), StateTransitionError>;
    fn get_display<'a>(&'a self) -> Ref<'a, Display>;
    fn get_step_title(&self) -> Ref<'_, str>;
    fn get_app_title(&self) -> &str;
}

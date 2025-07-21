pub mod control;
pub mod display;
pub mod error;
pub mod state;
pub mod transition;
pub mod variable_mapping;
pub mod workflow;

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

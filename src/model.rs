pub(crate) mod control;
pub(crate) mod display;
pub(crate) mod error;
pub(crate) mod state;
pub(crate) mod transition;
pub(crate) mod variable_mapping;
pub(crate) mod workflow;
pub use control::Control;
pub use display::{Display, Line};

pub trait TerminalFlow {
    fn run_control(&mut self, display_selection: &str, control: &Control);
    fn get_display(&self) -> &Display;
    fn get_controls(&self) -> Vec<Control>;
}

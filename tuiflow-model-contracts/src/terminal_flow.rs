use crate::control::{Control, Key};
use crate::display::Display;
use crate::error::StateTransitionError;

pub trait TerminalFlow {
    fn run_control(&mut self, display_selection_index: Option<usize>, key: &Key) -> Result<(), StateTransitionError>;
    fn get_display(&self) -> &Display;
    fn get_state_title(&self) -> String;
    fn get_app_title(&self) -> &str;
    fn get_state_controls(&self) -> Vec<Control>;
}

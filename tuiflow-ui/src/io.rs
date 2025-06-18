pub mod key_event_to_model_mapping;
pub mod sh_command_runner;

use tuiflow_model::TerminalFlow;
use tuiflow_model::control::Key;

pub trait InputUpdatedViewModel {
    type ViewState;
    fn needs_update(&self, state: &Self::ViewState, workflow: & impl TerminalFlow, key: &Key) -> bool; //TODO Make more generic over R,M
    fn update(&mut self, state: &mut Self::ViewState, workflow: &mut impl TerminalFlow, key: &Key);
}
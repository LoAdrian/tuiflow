pub mod key_event_to_model_mapping;

use tuiflow_model_contracts::control::Key;
use tuiflow_model_contracts::terminal_flow::TerminalFlow;

pub trait InputUpdatedViewModel {
    type ViewState;
    fn needs_update(&self, state: &Self::ViewState, workflow: & impl TerminalFlow, key: &Key) -> bool; //TODO Make more generic over R,M
    fn update(&mut self, state: &mut Self::ViewState, workflow: &mut impl TerminalFlow, key: &Key);
}
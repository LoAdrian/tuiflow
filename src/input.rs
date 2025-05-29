use crate::model::TerminalFlow;
use crate::model::control::Key;

pub(crate) trait InputUpdatedViewModel {
    type ViewState;
    fn needs_update(&self, state: &Self::ViewState, workflow: & impl TerminalFlow, key: &Key) -> bool; //TODO Make more generic over R,M
    fn update(&mut self, state: &mut Self::ViewState, workflow: &mut impl TerminalFlow, key: &Key);
}
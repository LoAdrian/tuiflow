use crate::{model::control::Key, workflow::ShCommandRunner, RegexVariableMapper, Workflow};

pub(crate) trait InputUpdatedViewModel {
    type ViewState;
    fn needs_update(&self, state: &Self::ViewState, workflow: &Workflow<ShCommandRunner, RegexVariableMapper>, key: &Key) -> bool; //TODO Make more generic over R,M
    fn update(&mut self, state: &mut Self::ViewState, workflow: &mut Workflow<ShCommandRunner, RegexVariableMapper>, key: &Key);
}
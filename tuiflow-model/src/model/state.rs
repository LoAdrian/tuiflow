mod state;
mod workflow_state;

use crate::model::variable::VariableSet;
use mockall::automock;
pub use state::*;
use tuiflow_model_contracts::control::Control;
use tuiflow_model_contracts::error::StateTransitionError;
pub use workflow_state::*;

#[automock]
pub trait Transit: Sized {
    fn run(&self, variables: &VariableSet) -> Option<Result<State<Self>, StateTransitionError>>;

    fn get_activation_control(&self) -> &Control;
}

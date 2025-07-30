mod workflow_state;
mod state;

use mockall::automock;
pub use workflow_state::*;
pub use state::*;
use tuiflow_model_contracts::control::Control;
use tuiflow_model_contracts::error::StateTransitionError;
use crate::model::variable::VariableSet;

#[automock]
pub trait Transit: Sized {
    fn run(
        &self,
        variables: &VariableSet,
    ) -> Result<State<Self>, StateTransitionError>;

    fn get_activation_control(&self) -> &Control;
}
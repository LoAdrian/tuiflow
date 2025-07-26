use crate::model::variable::VariableSet;
use crate::state::workflow_state::WorkflowState;
use crate::variable_mapping::VariableExtractor;
use crate::{Control, Display};
use std::cell::RefCell;
use std::rc::Rc;
use tuiflow_model_contracts::command_runner::CommandRunner;
use tuiflow_model_contracts::control::Key;
use tuiflow_model_contracts::error::StateTransitionError;

#[derive(Clone)]
pub struct State<R: CommandRunner, M: VariableExtractor> {
    workflow_state: Rc<RefCell<WorkflowState<R, M>>>,
    display: Display,
    arguments: Vec<VariableSet>,
}

impl<R: CommandRunner, M: VariableExtractor> State<R, M> {
    pub fn get_name(&self) -> String {
        self.workflow_state.borrow().get_display_name().to_string()
    }

    pub fn get_display(&self) -> &Display {
        &self.display
    }

    pub fn transition(
        &self,
        display_selection_index: Option<usize>,
        key: &Key,
    ) -> Result<State<R, M>, StateTransitionError> {
        let empty_set = VariableSet::empty();
        let variable_set = display_selection_index
            .map(|idx| {
                self.arguments
                    .get(idx)
                    .expect("Display selection index out of bounds")
            })
            .unwrap_or(&empty_set);

        self.workflow_state
            .borrow()
            .transition(variable_set, key)
    }

    pub fn get_controls(&self) -> Vec<Control> {
        self.workflow_state.borrow().get_controls()
    }

    pub fn new(
        workflow_state: Rc<RefCell<WorkflowState<R, M>>>,
        arguments: Vec<VariableSet>,
    ) -> Self {
        let display = workflow_state.borrow().get_display(&arguments);
        Self {
            workflow_state,
            display,
            arguments,
        }
    }
}

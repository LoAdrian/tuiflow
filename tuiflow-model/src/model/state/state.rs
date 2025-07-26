use tuiflow_model_contracts::control::Key;
use tuiflow_model_contracts::error::StateTransitionError;
use crate::model::variable::VariableSet;
use crate::state::workflow_state::WorkflowState;
use crate::variable_mapping::VariableExtractor;
use crate::{Control, Display};
use std::cell::RefCell;
use std::rc::Rc;
use tuiflow_model_contracts::command_runner::CommandRunner;

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

    pub fn transition(&self, display_selection_index: usize, key: &Key) -> Result<State<R, M>, StateTransitionError> {
        let selected_variable_set_opt = self.arguments.get(display_selection_index);
        if let Some(selected_variable_set) = selected_variable_set_opt {
            self.workflow_state.borrow().transition(selected_variable_set, key)
        } else {
            panic!("Display selection index out of bounds: {}", display_selection_index);
        }
    }
    
    pub fn get_controls(&self) -> Vec<Control> {
        self.workflow_state.borrow().get_controls()
    }

    pub fn new(workflow_state: Rc<RefCell<WorkflowState<R, M>>>, arguments: Vec<VariableSet>) -> Self {
        let display = workflow_state.borrow().get_display(&arguments);
        Self {
            workflow_state,
            display,
            arguments,
        }
    }
}

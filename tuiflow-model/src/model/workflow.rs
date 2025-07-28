use crate::model::variable::{Variable, VariableSet};
use crate::state::{State, Transit, WorkflowState};
use tuiflow_model_contracts::control::{Control, Key};
use tuiflow_model_contracts::display;
use tuiflow_model_contracts::error::{InitialTransitionError, StateTransitionError};
use tuiflow_model_contracts::terminal_flow::TerminalFlow;

pub struct Workflow<T: Transit> {
    current_state: State<T>,
    app_title: String,
}

impl<T: Transit> Workflow<T> {
    pub fn new(
        initializer_state: WorkflowState<T>,
        app_title: String,
    ) -> Result<Self, InitialTransitionError> {
        let init_control = initializer_state
            .get_controls()
            .pop()
            .expect("Initializer state must contain at least one control. Please report this issue on github.");
        let empty_variable_set: VariableSet = Vec::<Variable>::new().into_iter().collect();
        let current_state = initializer_state
            .transition(&empty_variable_set, &init_control.get_key())
            .map_err(|e| InitialTransitionError::from(e))?;
        Ok(Self {
            current_state,
            app_title,
        })
    }
}

impl<T: Transit> TerminalFlow for Workflow<T> {
    fn run_control(
        &mut self,
        display_selection_index: Option<usize>,
        key: &Key,
    ) -> Result<(), StateTransitionError> {
        let transition_result: Result<State<T>, StateTransitionError>;
        {
            transition_result = self.current_state.transition(display_selection_index, key);
        }

        match transition_result {
            Ok(next_state) => {
                self.current_state = next_state;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get_display(&self) -> &display::Display {
        self.current_state.get_display()
    }

    fn get_state_title(&self) -> String {
        self.current_state.get_name()
    }

    fn get_app_title(&self) -> &str {
        &self.app_title
    }

    fn get_state_controls(&self) -> Vec<Control> {
        self.current_state.get_controls()
    }
}

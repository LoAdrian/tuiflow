use std::{cell::{Ref, RefCell}, process::Command, rc::Rc};

use mockall::automock;

use super::{
    control::{Control, Key}, display::{self, Display}, error::StateTransitionError, state::State, transition::Transition, variable_mapping::VariableMapper, Line, TerminalFlow
};

pub(crate) mod builder;

pub(crate) struct Workflow<R: CommandRunner, M: VariableMapper> {
    current_state: Rc<RefCell<State<R, M>>>,
    app_title: String,
}

impl<R: CommandRunner, M: VariableMapper> Workflow<R, M> {
    pub fn new(
        initializer_state: State<R, M>,
        initial_selection: Line,
        app_title: String,
    ) -> Self {

        let init_control = initializer_state.get_controls().pop().expect("Initializer state must contain at least one control");
        let mut initializer_state_mut = initializer_state;
        let current_state = initializer_state_mut.transition(&initial_selection, &init_control.get_key())
            .expect("Something went wrong during the intial transition");
        Self {
            current_state,
            app_title
        }
    }
}

// TODO: Probably put this and impl to somewhere else
impl<R: CommandRunner, M: VariableMapper> TerminalFlow for Workflow<R, M> {
    fn run_control(&mut self, display_selection_index: usize, key: &Key) -> Result<(), StateTransitionError>{

        let transition_result: Result<Rc<RefCell<State<R, M>>>, StateTransitionError>; 
        {
            let selected_line: Line;

            { // scoped RefCell borrow
                // TODO move all this to state
                let display = self
                    .get_display();
                selected_line = display
                    .lines
                    .get(display_selection_index)
                    .expect("Invalid display selection index")
                    .clone(); // double borrow if we don't do this // TODO: eliminate this smell
            }

        
            transition_result = self.current_state.borrow_mut()
                .transition(&selected_line, key);
        }

        match transition_result {
            Ok(next_state) => {
                self.current_state = next_state;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    fn get_display<'a>(&'a self) -> Ref<'a, Display> {
        Ref::map(self.current_state.borrow(), |x| x.get_display())
    }

    fn get_step_title<'a>(&'a self) -> Ref<'_, str> {
        Ref::map(self.current_state.borrow(), |s| s.get_name())
    }

    fn get_app_title<'a>(&'a self) -> &str {
        &self.app_title
    }
}

#[automock]
pub trait CommandRunner {
    fn run_command(&self, command: &str) -> Result<String, ()>;
}

//move this out of the model
#[derive(Clone)]
pub struct ShCommandRunner;

impl CommandRunner for ShCommandRunner {
    fn run_command(&self, command: &str) -> Result<String, ()> {
        let cli_result = Command::new("sh").arg("-c").arg(command).output();
        if let Ok(cli_output) = cli_result {
            Ok(String::from_utf8(cli_output.stdout).unwrap())
        } else {
            Err(())
        }
    }
}

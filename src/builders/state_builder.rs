use std::{cell::RefCell, default, rc::Rc};

use crate::model::{state::{State, StateContext}, transition::Transition, variable_mapping::{self, VariableMapper}};

use super::TransitionBuilder;

#[derive(Clone)] 
pub struct StateBuilder<C: StateContext<M>, M: VariableMapper> {
    display_name: Option<String>,
    command_output_to_display_mapper: Option<M>,
    context: Option<Rc<RefCell<C>>>,
    transitions: Vec<Transition<C, M>>,
}

impl<C: StateContext<M>, M: VariableMapper> Default for StateBuilder<C, M> {
    fn default() -> Self {
        Self { 
            display_name: Default::default(),
            command_output_to_display_mapper: Default::default(),
            context: Default::default(),
            transitions: Default::default(),
        }
    }
}

impl<C: StateContext<M>, M: VariableMapper> StateBuilder<C, M> {
    pub fn new() -> Self { Default::default() }

    pub fn with_display_name(&mut self, display_name: String) -> &mut Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn with_command_output_to_display_mapper(&mut self, command_output_to_display_mapper: M) -> &mut Self {
        self.command_output_to_display_mapper = Some(command_output_to_display_mapper);
        self
    }

    pub fn with_context(&mut self, context: Rc<RefCell<C>>) -> &mut Self {
        self.context = Some(context);
        self
    }

    pub fn with_transitions(&mut self, transitions: Vec<Transition<C, M>>) -> &mut Self {
        self.transitions = transitions;
        self
    }

    pub fn add_transition(&mut self, transition: Transition<C, M>) -> &mut Self {
        self.transitions.push(transition);
        self
    }

    pub fn build_and_add_transition(&mut self, f: impl FnOnce(TransitionBuilder<C, M>) -> Transition<C, M>) -> &mut Self {
        let transition = f(TransitionBuilder::<C, M>::default());
        self.add_transition(transition)
    }

    pub fn build(&self) -> State<C, M> { // Consume self. Force clone(). Implies that clone is called on underlying types.
        State::new(
            self.display_name.as_ref().expect("Display name is required"), 
            self.command_output_to_display_mapper.as_ref().expect("Command output to display mapper is required").clone(),
            Rc::clone(self.context.as_ref().expect("Context is required")),
            self.transitions.clone(),
        )
    }
}
use std::{cell::RefCell, default, rc::Rc};

use crate::model::{state::{State, StateContext}, transition::Transition, variable_mapping::VariableMapper};

use super::TransitionBuilder;

#[derive(Clone)] // build() consumes self. Clone must be used to reuse the bulider. More explicit.
pub struct StateBuilder<C: StateContext> {
    display_name: Option<String>,
    command_output_to_display_mapper: Option<VariableMapper>,
    context: Option<Rc<RefCell<C>>>,
    transitions: Option<Vec<Transition<C>>>,
}

impl<C: StateContext> Default for StateBuilder<C> {
    fn default() -> Self {
        Self { 
            display_name: Default::default(),
            command_output_to_display_mapper: Default::default(),
            context: Default::default(),
            transitions: Default::default(),
        }
    }
}

impl<C: StateContext> StateBuilder<C> {
    pub fn new() -> Self { Default::default() }

    pub fn with_display_name(&mut self, display_name: String) -> &Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn with_command_output_to_display_mapper(&mut self, command_output_to_display_mapper: VariableMapper) -> &Self {
        self.command_output_to_display_mapper = Some(command_output_to_display_mapper);
        self
    }

    pub fn with_context(&mut self, context: &Rc<RefCell<C>>) -> &Self {
        self.context = Some(Rc::clone(context));
        self
    }

    pub fn with_transitions(&mut self, transitions: Vec<Transition<C>>) -> &Self {
        self.transitions = Some(transitions);
        self
    }

    pub fn add_transition(&mut self, transition: Transition<C>) -> &Self {
        if self.transitions.is_none() {
            self.transitions = Some(Vec::new());
        }

        self.transitions.as_mut().unwrap().push(transition);
        self
    }

    pub fn build_and_add_transition(&mut self, f: impl FnOnce(TransitionBuilder<C>) -> Transition<C>) -> &Self {
        let transition = f(TransitionBuilder::<C>::default());
        self.add_transition(transition)
    }

    pub fn build(self) -> State<C> { // Consume self. Force clone(). Implies that clone is called on underlying types.
        State::new(
            self.display_name.expect("Display name is required").as_str(), 
            self.command_output_to_display_mapper.expect("Command output to display mapper is required"),
            self.context.expect("Context is required"),
            self.transitions.expect("Transitions are required"),
        )
    }
}
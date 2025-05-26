use crate::app::configuration::AppConfiguration;
use crate::model::state::builder::StateBuilder;
use crate::model::state::State;
use crate::model::transition::builder::TransitionBuilder;
use crate::model::variable_mapping::RegexVariableMapper;
use crate::model::workflow::builder::WorkflowBuilder;
use crate::model::workflow::{ShCommandRunner, Workflow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct WorkflowFactory {}

impl WorkflowFactory {
    pub fn build_from_configuration(
        config: AppConfiguration,
    ) -> Workflow<ShCommandRunner, RegexVariableMapper> {
        let states: HashMap<String, Rc<RefCell<State<ShCommandRunner, RegexVariableMapper>>>> =
            config
                .states
                .iter()
                .map(|(name, config)| {
                    (
                        name.clone(),
                        Rc::new(RefCell::new(
                            StateBuilder::new()
                                .with_command_output_to_display_mapper(
                                    RegexVariableMapper::new(
                                        config.line_filter.as_str(),
                                        config.line_display_pattern.as_str(),
                                    )
                                    .unwrap(), //TODO: handle unwraps better
                                )
                                .with_display_name(name.clone())
                                .with_command_runner(ShCommandRunner)
                                .build(),
                        )),
                    )
                })
                .collect();

        for (name, state) in states.iter() {
            config
                .states
                .get(name)
                .unwrap()
                .transitions
                .iter()
                .for_each(|transition_config| {
                    let control = config
                        .controls
                        .custom_controls
                        .get(transition_config.control_name.as_str())
                        .unwrap(); // TODO
                    state.borrow_mut().add_transition(
                        control.get_key(),
                        TransitionBuilder::new()
                            .with_control(control.clone())
                            .with_next_state(
                                states
                                    .get(transition_config.next_state.as_str())
                                    .unwrap()
                                    .clone(),
                            )
                            .with_selected_display_to_command(RegexVariableMapper::new(transition_config.selection_filter.as_str(), transition_config.command_pattern.as_str()).unwrap())
                            .build(),
                    )
                });
        }

        WorkflowBuilder::new()
            .with_command_runner(ShCommandRunner)
            .with_initial_state(states.get(config.initial_state.as_str()).unwrap())
            .with_app_title(config.app_title.as_str())
            .with_initial_display_to_command_mapper(RegexVariableMapper::new("", config.initial_command.as_str()).unwrap())
            .build()
    }
}

use crate::app::configuration::AppConfiguration;
use crate::model::state::builder::StateBuilder;
use crate::model::state::State;
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
        let states: HashMap<String, Rc<RefCell<State<ShCommandRunner, RegexVariableMapper>>>> = config
            .states
            .into_iter()
            .map(|(name, config)| {
                (
                    name,
                    Rc::new(RefCell::new(
                        StateBuilder::new()
                            .with_command_output_to_display_mapper(
                                RegexVariableMapper::new(
                                    config.line_filter.as_str(),
                                    config.line_display_pattern.as_str(),
                                )
                                .unwrap(), //TODO: handle unwraps better
                            )
                            .build(),
                    )),
                )
            })
            .collect();

        WorkflowBuilder::new()
            .with_command_runner(ShCommandRunner)
            .with_initial_state(states.get(config.initial_state.as_str()).unwrap())
            .with_app_title(config.app_title.as_str())
            .build()
    }
}

use tuiflow_model::state::builder::StateBuilder;
use tuiflow_model::state::State;
use tuiflow_model::transition::builder::TransitionBuilder;
use tuiflow_model::variable_mapping::{RegexVariableMapper, VariableMapperCompilationError};
use tuiflow_model::workflow::builder::WorkflowBuilder;
use tuiflow_model::workflow::Workflow;
use eyre::OptionExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tuiflow_model_contracts::command_runner::CommandRunner;
use crate::configuration::AppConfiguration;

pub struct WorkflowFactory {}

impl WorkflowFactory {
    pub fn build_from_configuration<R: CommandRunner>(
        app_config: AppConfiguration,
    ) -> eyre::Result<Workflow<R, RegexVariableMapper>> {
        let states: Result<
            HashMap<String, Rc<RefCell<State<R, RegexVariableMapper>>>>,
            VariableMapperCompilationError,
        > = app_config
            .states
            .iter()
            .map(|(name, config)| {
                Self::build_state(
                    config.line_filter.as_str(),
                    config.line_display_pattern.as_str(),
                    name,
                )
                .map(|state| (name.clone(), state))
            })
            .collect();

        let states_unwrapped = states?;

        for (name, state) in states_unwrapped.iter() {
            let state_config = app_config.states.get(name).unwrap(); //safe unwrap
            for transition_config in &state_config.transitions {
                let transition_control = app_config
                    .controls
                    .custom_controls
                    .get(&transition_config.control_name)
                    .ok_or_eyre(format!(
                        "Control {} named in transition config not found",
                        transition_config.control_name
                    ))?;

                state.borrow_mut().add_transition(
                    transition_control.get_key(),
                    TransitionBuilder::new()
                        .with_control(transition_control.clone())
                        .with_next_state(
                            states_unwrapped
                                .get(transition_config.next_state.as_str())
                                .ok_or_eyre(format!(
                                    "Next state {} named in transition config not found",
                                    transition_config.next_state
                                ))?
                                .clone(),
                        )
                        .with_selected_display_to_command(RegexVariableMapper::new(
                            transition_config.selection_filter.as_str(),
                            transition_config.command_pattern.as_str(),
                        )?)
                        .build(),
                );
            }
        }

        let initial_state = states_unwrapped
            .get(app_config.initial_state.as_str())
            .ok_or_eyre(format!(
                "Initial state {} named in flow file not found in configuration",
                app_config.initial_state
            ))?
            .clone();

        let initial_command_mapper =
            RegexVariableMapper::new("", app_config.initial_command.as_str())?;

        Ok(WorkflowBuilder::new()
            .with_command_runner(R::new())
            .with_initial_state(&initial_state)
            .with_app_title(app_config.app_title.as_str())
            .with_initial_display_to_command_mapper(initial_command_mapper)
            .build())
    }

    fn build_state<R: CommandRunner>(
        line_filter: &str,
        line_display_pattern: &str,
        name: &str,
    ) -> Result<
        Rc<RefCell<State<R, RegexVariableMapper>>>,
        VariableMapperCompilationError,
    > {
        let variable_mapper = RegexVariableMapper::new(line_filter, line_display_pattern)?;
        Ok(Rc::new(RefCell::new(
            StateBuilder::new()
                .with_command_output_to_display_mapper(variable_mapper)
                .with_display_name(name.to_string())
                .with_command_runner(R::new())
                .build(),
        )))
    }
}

use crate::configuration::AppConfiguration;
use eyre::OptionExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tuiflow_model::state::WorkflowState;
use tuiflow_model::transition::Transition;
use tuiflow_model::variable_mapping::{RegexVariableExtractor, VariableInjector};
use tuiflow_model::workflow::Workflow;
use tuiflow_model::Control;
use tuiflow_model_contracts::command_runner::CommandRunner;
use tuiflow_model_contracts::control::Key;

pub struct WorkflowFactory {}

impl WorkflowFactory {
    pub fn build_from_configuration<R: CommandRunner>(
        app_config: AppConfiguration,
    ) -> eyre::Result<Workflow<R, RegexVariableExtractor>> {
        let states: HashMap<String, Rc<RefCell<WorkflowState<R, RegexVariableExtractor>>>> =
            app_config
                .states
                .iter()
                .map(|(name, config)| (name.clone(), Self::build_state(config.line_display_pattern.as_str(), name)))
                .collect();

        for (name, state) in states.iter() {
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

                let variable_extractor =
                    RegexVariableExtractor::new(transition_config.cli_output_variable_set_extractor.as_str())?;
                let transition = Transition::new(
                    transition_control.clone(),
                    states
                        .get(transition_config.next_state.as_str())
                        .ok_or_eyre(format!(
                            "Next state {} named in transition config not found",
                            transition_config.next_state
                        ))?
                        .clone(),
                    VariableInjector::new(transition_config.command_pattern.clone()),
                    R::new(),
                    variable_extractor,
                );
                state
                    .borrow_mut()
                    .add_transition(transition_control.get_key(), transition);
            }
        }

        let initial_state = states
            .get(app_config.initial_state.as_str())
            .ok_or_eyre(format!(
                "Initial state {} named in flow file not found in configuration",
                app_config.initial_state
            ))?
            .clone();

        let initial_transition = Transition::new(
            Control::new("INIT", Key::Backspace),
            initial_state.clone(),
            VariableInjector::new(app_config.initial_command.clone()),
            R::new(),
            RegexVariableExtractor::new(app_config.initial_cli_output_variable_set_extractor.as_str())?,
        );
        let initializer_state = WorkflowState::new(
            "INIT",
            VariableInjector::new("".to_string()),
            vec![initial_transition],
        );
        let workflow = Workflow::new(initializer_state, app_config.app_title)?;
        Ok(workflow)
    }

    fn build_state<R: CommandRunner>(
        line_display_pattern: &str,
        name: &str,
    ) -> Rc<RefCell<WorkflowState<R, RegexVariableExtractor>>> {
        let variable_mapper = VariableInjector::new(line_display_pattern.to_string());
        let state = WorkflowState::<R, RegexVariableExtractor>::new(name, variable_mapper, vec![]);
        Rc::new(RefCell::new(state))
    }
}

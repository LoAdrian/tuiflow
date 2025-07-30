use tuiflow_model_contracts::control::Key;
use tuiflow_model::Control;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppConfiguration {
    pub app_title: String,
    pub controls: ControlsConfiguration,
    pub initial_command: String,
    pub initial_state: String,
    pub initial_cli_output_variable_set_extractor: String,
    pub states: HashMap<String, StateConfiguration>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StateConfiguration {
    pub transitions: Vec<TransitionConfiguration>,
    pub line_display_pattern: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TransitionConfiguration {
    pub control_name: String,
    pub cli_output_variable_set_extractor: String,
    pub command_pattern: String,
    pub next_state: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ControlsConfiguration {
    pub selection_up: Control,
    pub selection_down: Control,
    pub quit: Control,
    pub custom_controls: HashMap<String, Control>,
}

impl Default for ControlsConfiguration {
    fn default() -> Self {
        Self {
            selection_up: Control::new("selection up", Key::Char('k')),
            selection_down: Control::new("selection down", Key::Char('j')),
            quit: Control::new("quit", Key::Char('q')),
            custom_controls: HashMap::new(),
        }
    }
}
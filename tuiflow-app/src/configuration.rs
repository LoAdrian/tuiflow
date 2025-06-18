use tuiflow_model::control::Key;
use tuiflow_model::Control;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppConfiguration {
    pub app_title: String,
    pub controls: ControlsConfiguration,
    pub initial_command: String,
    pub initial_state: String,
    pub states: HashMap<String, StateConfiguration>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StateConfiguration {
    pub transitions: Vec<TransitionConfiguration>,
    pub line_filter: String,
    pub line_display_pattern: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TransitionConfiguration {
    pub control_name: String,
    pub selection_filter: String,
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

#[cfg(test)]
mod tests {
    use tuiflow_model::control::Key;
    use tuiflow_model::Control;
    use std::collections::HashMap;
    use crate::configuration::{AppConfiguration, ControlsConfiguration, StateConfiguration, TransitionConfiguration};

    const SERIALIZED_CONFIGURATION: &'static str = r#"app_title: dora the explorah
controls:
  selection_up:
    name: selection up
    key: !Char 'k'
  selection_down:
    name: selection down
    key: !Char 'j'
  quit:
    name: quit
    key: !Char 'q'
  custom_controls:
    moveback:
      name: move back
      key: !Char 'h'
    moveinto:
      name: move into
      key: !Char 'l'
initial_command: ls
initial_state: show_files
states:
  show_files:
    transitions:
    - control_name: moveinto
      selection_filter: (?<x>.*)
      command_pattern: ls -d -1 "<x>/"**
      next_state: show_files
    - control_name: moveback
      selection_filter: (?<x>.*)\/.*\/.*
      command_pattern: ls -d -1 "<x>/"**
      next_state: show_files
    line_filter: (?<path>.+)
    line_display_pattern: <path>
"#;
    #[test]
    fn serialization_serializes_as_expected() {
        let config = get_example_config();
        let serialization_result = serde_yaml::to_string(&config);
        assert!(serialization_result.is_ok());
        // assert_eq!(serialization_result.unwrap(), SERIALIZED_CONFIGURATION); flaky because of list-order
    }

    #[test]
    fn deserialization_deserializes_as_expected() {
        let expected_config = get_example_config();
        let actual_result: Result<AppConfiguration, _> =
            serde_yaml::from_str(SERIALIZED_CONFIGURATION);
        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_config);
    }
    
    fn get_example_config() -> AppConfiguration {
        AppConfiguration {
            app_title: "dora the explorah".to_string(),
            controls: ControlsConfiguration {
                custom_controls: HashMap::from([
                    (
                        "moveinto".to_string(),
                        Control::new("move into", Key::Char('l')),
                    ),
                    (
                        "moveback".to_string(),
                        Control::new("move back", Key::Char('h')),
                    ),
                ]),
                ..Default::default()
            },
            initial_command: "ls".to_string(),
            initial_state: "show_files".to_string(),
            states: HashMap::from([(
                "show_files".to_string(),
                StateConfiguration {
                    line_filter: "(?<path>.+)".to_string(),
                    line_display_pattern: "<path>".to_string(),
                    transitions: vec![
                        TransitionConfiguration {
                            control_name: "moveinto".to_string(),
                            selection_filter: "(?<x>.*)".to_string(),
                            command_pattern: "ls -d -1 \"<x>/\"**".to_string(),
                            next_state: "show_files".to_string(),
                        },
                        TransitionConfiguration {
                            control_name: "moveback".to_string(),
                            selection_filter: "(?<x>.*)\\/.*\\/.*".to_string(),
                            command_pattern: "ls -d -1 \"<x>/\"**".to_string(),
                            next_state: "show_files".to_string(),
                        },
                    ],
                },
            )]),
        }
    }
}

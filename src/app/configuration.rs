use crate::model::control::Key;
use crate::model::Control;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct AppConfiguration {
    pub app_title: String,
    pub controls: ControlsConfiguration,
    pub  initial_command: String,
    pub  states: HashMap<String, StateConfiguration>,

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct StateConfiguration {
    pub  transitions: Vec<TransitionConfiguration>,
    pub line_filter: String,
    pub line_display_pattern: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct TransitionConfiguration {
    pub  control_name: String,
    pub  selection_filter: String,
    pub  command_pattern: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct ControlsConfiguration {
    pub  selection_up: Control,
    pub  selection_down: Control,
    pub  quit: Control,
    pub  custom_controls: HashMap<String, Control>,
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
    use crate::app::configuration::{AppConfiguration, ControlsConfiguration, StateConfiguration, TransitionConfiguration};
    use crate::model::control::Key;
    use crate::model::Control;
    use std::collections::HashMap;

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
    moveinto:
      name: move into
      key: !Char 'l'
    moveback:
      name: move back
      key: !Char 'h'
initial_command: ls
states:
  show files:
    transitions:
    - control_name: moveinto
      selection_filter: (?<x>.*)
      command_pattern: ls -d -1 "<x>/"**
    - control_name: moveback
      selection_filter: (?<x>.*)\/.*\/.*
      command_pattern: ls -d -1 "<x>/"**
    line_filter: (?<path>.+)
    line_display_pattern: <path>
"#;
    #[test]
    fn serialization_serializes_as_expected() {
        let config = AppConfiguration {
            app_title: "dora the explorah".to_string(),
            controls: ControlsConfiguration {
                custom_controls: HashMap::from([
                    (
                        "moveback".to_string(),
                        Control::new("move back", Key::Char('h')),
                    ),
                    (
                        "moveinto".to_string(),
                        Control::new("move into", Key::Char('l')),
                    ),
                ]),
                ..Default::default()
            },
            initial_command: "ls".to_string(),
            states: HashMap::from([(
                "show files".to_string(),
                StateConfiguration {
                    line_filter: "(?<path>.+)".to_string(),
                    line_display_pattern: "<path>".to_string(),
                    transitions: vec![TransitionConfiguration {
                        control_name: "moveinto".to_string(),
                        selection_filter: "(?<x>.*)".to_string(),
                        command_pattern: "ls -d -1 \"<x>/\"**".to_string(),
                    },
                                      TransitionConfiguration {
                                          control_name: "moveback".to_string(),
                                          selection_filter: "(?<x>.*)\\/.*\\/.*".to_string(),
                                          command_pattern: "ls -d -1 \"<x>/\"**".to_string(),
                                      }],
                },
            )]),
        };
        let serialization_result = serde_yaml::to_string(&config);
        assert!(serialization_result.is_ok());
        assert_eq!(serialization_result.unwrap(), SERIALIZED_CONFIGURATION);
    }

    #[test]
    fn deserialization_deserializes_as_expected() {
        let expected_config = AppConfiguration {
            app_title: "dora the explorah".to_string(),
            controls: ControlsConfiguration {
                custom_controls: HashMap::from([
                    (
                        "moveback".to_string(),
                        Control::new("move back", Key::Char('h')),
                    ),
                    (
                        "moveinto".to_string(),
                        Control::new("move into", Key::Char('l')),
                    ),
                ]),
                ..Default::default()
            },
            initial_command: "ls".to_string(),
            states: HashMap::from([(
                "show files".to_string(),
                StateConfiguration {
                    line_filter: "(?<path>.+)".to_string(),
                    line_display_pattern: "<path>".to_string(),
                    transitions: vec![TransitionConfiguration {
                        control_name: "moveinto".to_string(),
                        selection_filter: "(?<x>.*)".to_string(),
                        command_pattern: "ls -d -1 \"<x>/\"**".to_string(),
                    },
                                      TransitionConfiguration {
                                          control_name: "moveback".to_string(),
                                          selection_filter: "(?<x>.*)\\/.*\\/.*".to_string(),
                                          command_pattern: "ls -d -1 \"<x>/\"**".to_string(),
                                      }],
                },
            )]),
        };
        let actual_result: Result<AppConfiguration, _> = serde_yaml::from_str(SERIALIZED_CONFIGURATION);
        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_config);
    }
}
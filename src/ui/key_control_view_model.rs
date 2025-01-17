use crate::model::Control;


// TODO: At some point implement key-combinations instead of single-key controls
pub struct KeyControlViewModel {
    control: Control,
}

impl KeyControlViewModel {
    pub fn new(control: Control) -> Self {
        Self { control }
    }
}

impl From<&KeyControlViewModel> for String {
    fn from(value: &KeyControlViewModel) -> Self {
        format!("{}: {}", value.control.get_key(), value.control.get_name())
    }
}
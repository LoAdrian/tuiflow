use crate::model::Control;


// TODO: At some point implement key-combinations instead of single-key controls
#[derive(Clone)]
pub struct KeyControlViewModel<'a> {
    control: &'a Control,
}

impl<'a> KeyControlViewModel<'a> {
    pub fn new(control: &'a Control) -> Self {
        Self { control }
    }
}

impl<'a> From<KeyControlViewModel<'a>> for &'a str {
    fn from(value: KeyControlViewModel) -> Self {
        format!("{}: {}", value.control.get_key(), value.control.get_name())
    }
}
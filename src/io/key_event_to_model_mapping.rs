use std::error::Error;
use std::fmt::{Display, Formatter};
use crossterm::event::{KeyCode, KeyEvent};
use crate::model::control::Key;

pub(crate) fn key_event_to_key(event: &KeyEvent) -> Result<Key, KeyEventToKeyMappingError> {
    let key = match event.code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Enter => Key::Enter,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Tab => Key::Tab,
        KeyCode::Esc => Key::Esc,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Delete => Key::Delete,
        KeyCode::Insert => Key::Insert,
        KeyCode::F(n) => Key::F(n),
        _ => return Err(KeyEventToKeyMappingError),
    };

    Ok(key)
}

#[derive(Debug)]
pub(crate) struct KeyEventToKeyMappingError;

impl Display for KeyEventToKeyMappingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to map KeyEvent to Key. This is likely a development oversight. Please report this issue on github.")
    }
}

impl Error for KeyEventToKeyMappingError {}

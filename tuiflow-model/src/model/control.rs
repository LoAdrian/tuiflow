use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Control {
    name: String,
    key: Key,
}

impl<'a> Control {
    pub fn new(name: &str, key: Key) -> Self {
        Self {
            name: String::from(name),
            key,
        }
    }

    pub fn get_key(&self) -> Key {
        self.key
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.name)
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Tab,
    Esc,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Char(c) => write!(f, "{}", c),
            Key::Enter => write!(f, "Enter"),
            Key::Backspace => write!(f, "Backspace"),
            Key::Tab => write!(f, "Tab"),
            Key::Esc => write!(f, "Esc"),
            Key::Up => write!(f, "Up"),
            Key::Down => write!(f, "Down"),
            Key::Left => write!(f, "Left"),
            Key::Right => write!(f, "Right"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PageUp"),
            Key::PageDown => write!(f, "PageDown"),
            Key::Delete => write!(f, "Delete"),
            Key::Insert => write!(f, "Insert"),
            Key::F(n) => write!(f, "F{}", n),
        }
    }
}



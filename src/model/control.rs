use std::fmt::Display;

#[derive(Clone)]
#[derive(PartialEq, Debug)]
pub struct Control {
    name: String,
    key: Key,
}

impl<'a> Control {
    pub fn new(name: &str, key: Key) -> Self {
        Self {
            name: String::from(name),
            key: key,
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
pub enum Key {
    Char(char),
    SpecialKey(SpecialKey),
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Char(c) => write!(f, "{}", c),
            Key::SpecialKey(s) => write!(f, "{}", s.to_string()),
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpecialKey {
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

impl Display for SpecialKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialKey::Enter => write!(f, "Enter"),
            SpecialKey::Backspace => write!(f, "Backspace"),
            SpecialKey::Tab => write!(f, "Tab"),
            SpecialKey::Esc => write!(f, "Esc"),
            SpecialKey::Up => write!(f, "Up"),
            SpecialKey::Down => write!(f, "Down"),
            SpecialKey::Left => write!(f, "Left"),
            SpecialKey::Right => write!(f, "Right"),
            SpecialKey::Home => write!(f, "Home"),
            SpecialKey::End => write!(f, "End"),
            SpecialKey::PageUp => write!(f, "PageUp"),
            SpecialKey::PageDown => write!(f, "PageDown"),
            SpecialKey::Delete => write!(f, "Delete"),
            SpecialKey::Insert => write!(f, "Insert"),
            SpecialKey::F(n) => write!(f, "F{}", n),
        }
    }
}
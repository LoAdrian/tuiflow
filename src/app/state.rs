use crate::{model::control::Key};

pub(crate) enum AppState {
    Running,
    Quitting,
}

impl AppState {
    pub fn quit(&mut self) {
        *self = Self::Quitting;
    }

    pub fn is_running(&self) -> bool {
        match self {
            Self::Running => true,
            Self::Quitting => false,
        }
    }

    pub fn update(&mut self, key: Key) {
        match key {
            Key::Char('q') => self.quit(),
            _ => (),
        }
    }
}
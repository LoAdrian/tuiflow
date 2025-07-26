use tuiflow_model_contracts::control::{Control, Key};

pub enum AppState {
    Running { quit_control: Control },
    Quitting,
}

impl AppState {
    pub fn quit(&mut self) {
        *self = Self::Quitting;
    }

    pub fn is_running(&self) -> bool {
        match self {
            Self::Running{quit_control: _} => true,
            Self::Quitting => false,
        }
    }

    pub fn update(&mut self, key: Key) {
        if let Self::Running { quit_control } = self {
            if key == quit_control.get_key() {
                self.quit();
            }
        }
    }
}

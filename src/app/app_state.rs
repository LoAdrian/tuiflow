use std::{cell::RefCell, rc::Rc};

use crate::{input::{ObservableKeyboard, Observer}, model::control::Key};

pub(crate) enum AppState {
    Running,
    Quitting,
}

impl AppState {
    pub fn new(keyboard_observable: &mut ObservableKeyboard) -> Rc<RefCell<Self>> {
        let _self = Rc::new(RefCell::new(Self::Running));
        keyboard_observable.register(Key::Char('q'), _self.clone());        
        _self
    }
    pub fn quit(&mut self) {
        *self = Self::Quitting;
    }

    pub fn is_running(&self) -> bool {
        match self {
            Self::Running => true,
            Self::Quitting => false,
        }
    }
}

impl Observer for AppState {
    fn update(&mut self, key: Key) {
        match key {
            Key::Char('q') => self.quit(),
            _ => (),
        }
    }
}
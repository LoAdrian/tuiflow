use std::{cell::RefCell, collections::HashMap, rc::Rc};

use mockall::automock;

use crate::model::control::Key;

pub struct ObservableKeyboard {
    observers_by_key: HashMap<Key, Vec<Rc<RefCell<dyn Observer>>>>
}

impl ObservableKeyboard {
    pub fn new() -> Self {
        Self {
            observers_by_key: HashMap::new()
        }
    }

    pub fn register(&mut self, key: Key, observer: Rc<RefCell<dyn Observer>>) {
        self.observers_by_key.entry(key).or_default().push(observer);
    }

    pub fn notify_observers(&mut self, key: Key) {
        for observer in self.observers_by_key.entry(key).or_default() {
            observer.borrow_mut().update(key);
        }
    }
}


#[automock]
pub trait Observer {
    fn update(&mut self, key: Key);
}

#[cfg(test)]
mod observable_keyboard_tests {
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn notify_observers_with_single_observer_registered_calls_update() {
        // Arrange
        let mut observable_keyboard = ObservableKeyboard::new();
        let mock_observer = Rc::new(RefCell::new(MockObserver::new()));

        mock_observer.borrow_mut().expect_update()
            .with(eq(Key::Char('a')))
            .once()
            .returning(|_| ());
        observable_keyboard.register(Key::Char('a'), mock_observer.clone());
        
        // Act
        observable_keyboard.notify_observers(Key::Char('a'));
    }

    #[test]
    fn notify_observers_with_observer_registered_to_wrong_key_doe_not_call_update() {
        // Arrange
        let mut observable_keyboard = ObservableKeyboard::new();
        let mock_observer = Rc::new(RefCell::new(MockObserver::new()));

        mock_observer.borrow_mut().expect_update()
            .with(eq(Key::Char('a')))
            .times(0)
            .returning(|_| ());
        observable_keyboard.register(Key::Char('a'), mock_observer.clone());
        
        // Act
        observable_keyboard.notify_observers(Key::Char('b'));
    }
    
    #[test]
    fn notify_observers_with_multiple_observers_registered_calls_all_observers() {
        // Arrange
        let mut observable_keyboard = ObservableKeyboard::new();
        let mock_observer_1 = Rc::new(RefCell::new(MockObserver::new()));
        let mock_observer_2 = Rc::new(RefCell::new(MockObserver::new()));

        mock_observer_1.borrow_mut().expect_update()
            .with(eq(Key::Char('a')))
            .once()
            .returning(|_| ());
        mock_observer_2.borrow_mut().expect_update()
            .with(eq(Key::Char('a')))
            .once()
            .returning(|_| ());

        observable_keyboard.register(Key::Char('a'), mock_observer_1.clone());
        observable_keyboard.register(Key::Char('a'), mock_observer_2.clone());
        
        // Act
        observable_keyboard.notify_observers(Key::Char('a'));
    }
}
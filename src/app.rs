use std::{cell::RefCell, rc::Rc, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use eyre::{Context, Result};
use ratatui::{
    widgets::WidgetRef, DefaultTerminal, Frame
};

use crate::{
    input::{ObservableKeyboard, Observer}, model::{control::Key, variable_mapping::RegexVariableMapper, workflow::{builder::WorkflowBuilder, ShCommandRunner, Workflow}, Control}, ui::{
        body::BodyWidget, key_control_view_model::KeyControlViewModel, legend_widget::LegendWidget, main_widget::{MainWidget, MainWidgetBuilder}, titlebar_widget::{TitleBarWidget, TitleBarWidgetBuilder}
    }
};

pub enum AppState {
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

pub struct App {
    app_state: Rc<RefCell<AppState>>,
    workflow_state: Workflow<ShCommandRunner, RegexVariableMapper>,
    keyboard_observable: ObservableKeyboard,
    main_widget: MainWidget,
}

impl App {
    pub fn new() -> Self {
        let legend_entries = vec![
            KeyControlViewModel::new(Control::new("Do shit", Key::Char('a'))),
            KeyControlViewModel::new(Control::new("Do shit", Key::Char('a'))),
            KeyControlViewModel::new(Control::new("Do shit", Key::Char('a'))),
            KeyControlViewModel::new(Control::new("Do shit", Key::Char('a'))),
        ];
        
        
        let mut keyboard_observable = ObservableKeyboard::new();
        let app_state = AppState::new(&mut keyboard_observable);
        
        let main_widget = MainWidget::new(
            TitleBarWidget::new("app_title", "state_title"),
            BodyWidget::new(&mut keyboard_observable, Key::Char('k'), Key::Char('j'), vec!["x".to_string(), "y".to_string(), "z".to_string()]),
            LegendWidget::new(legend_entries),
        );
        let command_runner = ShCommandRunner;
        let workflow_state = Workflow::new(command_runner);
        Self {
            app_state,
            workflow_state,
            keyboard_observable,
            main_widget
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.app_state.borrow().is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.update()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        self.main_widget.render_ref(frame.area(), frame.buffer_mut());
    }

    fn update(&mut self) -> Result<()> { // TODO make input handling more sophisticated and put in into input module
        if event::poll(Duration::from_millis(250)).context("failed to poll event")? {
            if let Event::Key(key_event) = event::read().context("failed to read event")? {
                let key = key_event_to_key(&key_event)?;
                self.keyboard_observable.notify_observers(key);
            }
        }
        Ok(())
    }
}

fn key_event_to_key(event: &KeyEvent) -> Result<Key> {
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
        _ => return Err(eyre::eyre!("Unsupported key event: {:?}", event)),
    };
    
    Ok(key)
}

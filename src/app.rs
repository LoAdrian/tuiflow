use crate::{
    input::InputUpdatedViewModel,
    model::{control::Key, Control},
    ui::main_widget::{MainState, MainViewModel, MainWidget},
    workflow::ShCommandRunner,
    RegexVariableMapper, Workflow,
};
use app_state::AppState;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use eyre::{Context, Result};
use ratatui::{widgets::StatefulWidgetRef, DefaultTerminal, Frame};
use std::time::Duration;

mod app_state;

// TODO: App is the actual entry point -> not the main function
// App is also not the user interface
pub struct App {
    app_state: AppState,
    quit_control: Control,
}

impl App {
    pub fn new(quit_control: Control) -> Self {
        Self {
            app_state: AppState::Running,
            quit_control,
        }
    }

    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        mut workflow: Workflow<ShCommandRunner, RegexVariableMapper>,
    ) {
        let mut view_model = MainViewModel::new(
            &workflow,
            Control::new("Select up", Key::Char('k')),
            Control::new("Select down", Key::Char('j')),
        );
        let mut main_widget = MainWidget::new(&view_model);
        let mut main_state = MainState::new();

        while self.app_state.is_running() {
            if let Some(key) = self.should_update(&workflow, &view_model, &main_state) {
                self.update(&mut view_model, &mut main_state, &mut workflow, &key);
                main_widget = MainWidget::new(&view_model)
            }
            _ = terminal.draw(|frame| self.draw(frame, &main_widget, &mut main_state));
        }
    }

    fn draw(&mut self, frame: &mut Frame, main_widget: &MainWidget, state: &mut MainState) {
        main_widget.render_ref(frame.area(), frame.buffer_mut(), state);
    }

    fn should_update(&mut self, workflow: &Workflow<ShCommandRunner, RegexVariableMapper>, view_model: &MainViewModel, state: &MainState) -> Option<Key> {
        if event::poll(Duration::from_millis(250))
            .context("failed to poll event")
            .unwrap()
        {
            if let Event::Key(key_event) = event::read().context("failed to read event").unwrap() {
                let key = key_event_to_key(&key_event).unwrap();
                if key == self.quit_control.get_key() {
                    return Some(self.quit_control.get_key());
                }

                if view_model.needs_update(state, &workflow, &key) {
                    return Some(key);
                }
            }
        }
        None
    }

    fn update(&mut self, view_model: &mut MainViewModel, state: &mut MainState, workflow: &mut Workflow<ShCommandRunner, RegexVariableMapper>, key: &Key) {
        if (*key == Key::Char('q')) {
            self.app_state.quit();
            return;
        }
        view_model.update(state, workflow, &key);
    }
}

//TODO move this to IO maybe
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

use std::{cell::RefCell, rc::Rc, time::Duration};

use app_state::AppState;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use eyre::{Context, Result};
use ratatui::{
    widgets::StatefulWidgetRef, DefaultTerminal, Frame
};

use crate::{
    input::ObservableKeyboard, model::{control::Key, Control, TerminalFlow}, ui::{
        body::{BodyState, BodyWidget}, key_control_view_model::KeyControlViewModel, legend_widget::LegendWidgetBuilder, main_widget::{MainViewModel, MainWidget,
    }, titlebar_widget::TitleBarWidget
}};

mod app_state;

// TODO: App is the actual entry point -> not the main function
// App is also not the user interface
pub struct App<TWorkFlow: TerminalFlow> {
    app_state: Rc<RefCell<AppState>>,
    view_model: Rc<RefCell<MainViewModel<TWorkFlow>>>,
    keyboard_observable: ObservableKeyboard,
    main_widget: MainWidget<TWorkFlow>,
}

impl<TWorkflow: TerminalFlow + 'static> App<TWorkflow> {
    pub fn new(workflow: TWorkflow, all_controls: Vec<Control>, select_down: Control, select_up: Control) -> Self {
        
        let mut keyboard_observable = ObservableKeyboard::new();

        let app_state = AppState::new(&mut keyboard_observable);

        let body_view_model = BodyState::new(&mut keyboard_observable, select_up.get_key(), select_down.get_key());

        let list_content = workflow.get_display().lines.iter().map(|line| { line.0.to_string() }).collect();

        let view_model = Rc::new(RefCell::new(MainViewModel::new(workflow, body_view_model)));
        all_controls.iter().for_each(|c: &Control| { keyboard_observable.register(c.get_key(), view_model.clone()); });
        

        let legend_entries = all_controls.iter().map(|control| {
            KeyControlViewModel::new(control.clone())
        }).collect::<Vec<_>>();

        let main_widget = MainWidget::new(

            TitleBarWidget::new(workflow.get_app_title().to_string(), workflow.get_step_title().to_string()),
            BodyWidget::new(list_content),
            LegendWidgetBuilder::new()
                .with_entries(legend_entries.clone())
                .build());



        Self {
            app_state,
            view_model,
            keyboard_observable,
            main_widget,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.app_state.borrow().is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.update()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.main_widget.render_ref(frame.area(), frame.buffer_mut(), &mut self.view_model);
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

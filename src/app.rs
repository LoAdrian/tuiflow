use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use eyre::{Context, Result};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};

use crate::{
    model::Control,
    ui::{
        body_widget::BodyWidget, key_control_view_model::KeyControlViewModel, legend_widget::{LegendWidget, WIDGET_PADDING_VERTICAL}, titlebar_widget::TitleBarWidget
    },
};

pub struct App {
    is_running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { is_running: true }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.update()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let legend_entries = vec![
            KeyControlViewModel::new(Control::new("Do shit", "a")),
            KeyControlViewModel::new(Control::new("Do shit", "a")),
            KeyControlViewModel::new(Control::new("Do shit", "a")),
            KeyControlViewModel::new(Control::new("Do shit", "a")),
        ];
        let legened_row_count =
            (legend_entries.len() as f32 / 3.0).ceil() as u16 + WIDGET_PADDING_VERTICAL;
        let layout = Layout::vertical([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(legened_row_count),
        ]);
        let [title_bar, body, legend] = layout.areas(frame.area());
        frame.render_widget_ref(TitleBarWidget::new("Hello World", "Step one"), title_bar);
        let mut state = "asdf".to_string();
        frame.render_stateful_widget_ref(BodyWidget::new(), body, &mut state);
        frame.render_widget_ref(LegendWidget::new(legend_entries), legend);
    }

    fn update(&mut self) -> Result<()> {
        self.is_running = !should_quit()?;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.is_running
    }
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("failed to poll event")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}

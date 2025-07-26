use std::time::Duration;
use crossterm::event;
use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::StatefulWidgetRef;
use tuiflow_model::Control;
use tuiflow_model_contracts::control::Key;
use tuiflow_model::variable_mapping::RegexVariableExtractor;
use tuiflow_model::workflow::Workflow;
use tuiflow_model_contracts::command_runner::CommandRunner;
use tuiflow_ui::io;
use tuiflow_ui::io::InputUpdatedViewModel;
use tuiflow_ui::main_widget::{MainState, MainViewModel, MainWidget};
use crate::configuration::AppConfiguration;
use crate::factory::WorkflowFactory;
use crate::state::AppState;

pub mod configuration;
mod factory;
mod state;

pub struct App<R: CommandRunner> {
    app_state: AppState,
    quit_control: Control,
    workflow: Workflow<R, RegexVariableExtractor>,
}

impl<R: CommandRunner> App<R> {
    pub fn new(configuration: AppConfiguration) -> eyre::Result<Self> {
        let quit_control = configuration.controls.quit.clone();
        let workflow = WorkflowFactory::build_from_configuration(configuration)?;
        Ok(Self {
            app_state: AppState::Running,
            workflow,
            quit_control,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        let mut view_model = MainViewModel::new(
            &self.workflow,
            Control::new("Select up", Key::Char('k')),
            Control::new("Select down", Key::Char('j')),
        );
        let mut main_widget = MainWidget::new(&view_model);
        let mut main_state = MainState::new();

        while self.app_state.is_running() {
            if let Some(key) = self.should_update(&view_model, &main_state)? {
                self.update(&mut view_model, &mut main_state, &key);
                main_widget = MainWidget::new(&view_model)
            }
            _ = terminal.draw(|frame| self.draw(frame, &main_widget, &mut main_state));
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, main_widget: &MainWidget, state: &mut MainState) {
        main_widget.render_ref(frame.area(), frame.buffer_mut(), state);
    }

    fn should_update(
        &mut self,
        view_model: &MainViewModel,
        state: &MainState,
    ) -> eyre::Result<Option<Key>> {
        if event::poll(Duration::from_millis(250))? { //TODO: Move to io maybe?
            {
                if let Event::Key(key_event) = event::read()? {
                    let key = io::key_event_to_model_mapping::key_event_to_key(&key_event)?;
                    self.app_state.update(key.clone());

                    if view_model.needs_update(state, &self.workflow, &key) {
                        return Ok(Some(key));
                    }
                }
            }
        }
        Ok(None)
    }

    fn update(&mut self, view_model: &mut MainViewModel, state: &mut MainState, key: &Key) {
        if *key == self.quit_control.get_key() {
            self.app_state.quit();
            return;
        }
        view_model.update(state, &mut self.workflow, &key);
    }
}
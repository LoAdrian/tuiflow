use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::{StatefulWidgetRef, WidgetRef}};


use crate::{input::InputUpdatedViewModel, model::{control::Key, Control, TerminalFlow}, workflow::{self, ShCommandRunner}, RegexVariableMapper, Workflow};

use super::{body::{BodyState, BodyViewModel, BodyWidget}, controls_widget::{ControlsViewModel, ControlsWidget, WIDGET_PADDING_VERTICAL}, key_control_view_model::KeyControlViewModel, titlebar_widget::{TitleBarViewModel, TitleBarWidget}};

pub(crate) struct MainWidget<'a> {
    title_bar: TitleBarWidget<'a>,
    body: BodyWidget<'a>,
    legend_footer: ControlsWidget<'a>,
}

impl<'a> MainWidget<'a> {
    pub fn new(view_model: &'a MainViewModel) -> Self {
        let title_bar = TitleBarWidget::new(&view_model.titlebar_view_model);
        let body = BodyWidget::new(&view_model.body_view_model);
        let legend_footer = ControlsWidget::new(&view_model.legend_view_model);

        Self {
            title_bar: title_bar,
            body: body,
            legend_footer: legend_footer,
        }
    }
}

impl<'a> StatefulWidgetRef for MainWidget<'a> {
    type State = MainState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let legened_row_count =
            (self.legend_footer.get_legend_size() as f32 / 3.0).ceil() as u16 + WIDGET_PADDING_VERTICAL;
        let layout = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(legened_row_count),
        ]);
        let [title_bar, body, legend] = layout.areas(area);
        self.title_bar.render_ref(title_bar, buf);
        self.body.render_ref(body, buf, &mut state.body_state);
        self.legend_footer.render_ref(legend, buf)
    }
    
}

pub(crate) struct MainViewModel {
    titlebar_view_model: TitleBarViewModel,
    body_view_model: BodyViewModel,
    legend_view_model: ControlsViewModel,
}

impl MainViewModel {
    pub fn new(workflow: &Workflow<ShCommandRunner, RegexVariableMapper>, selection_up: Control, selection_down: Control) -> Self {
        let display = workflow.get_display().clone();
        let select_up_key = selection_up.get_key();
        let select_down_key = selection_down.get_key();
        Self {
            titlebar_view_model: TitleBarViewModel::new(workflow),
            body_view_model: BodyViewModel::new(display, select_up_key, select_down_key),
            legend_view_model: ControlsViewModel::new(workflow, selection_up, selection_down),
        }
    }
}

//TODO Make this all more condiitional: e.g. only recreate part x when part x actually changes
impl InputUpdatedViewModel for MainViewModel {
    type ViewState = MainState;
    fn update(&mut self, state: &mut Self::ViewState, workflow: &mut Workflow<ShCommandRunner, RegexVariableMapper>, key: &Key) {
        if workflow.get_state_controls().iter().any(|control| control.get_key() == *key) {
            _ = workflow.run_control(state.body_state.get_selected_line_index(), key);
        }
        self.body_view_model.update(&mut state.body_state, workflow, key);
        self.legend_view_model.update(&mut (), workflow, key);
        self.titlebar_view_model.update(&mut (), workflow, key);
    }
    
    fn needs_update(&self, state: &Self::ViewState, workflow: &Workflow<ShCommandRunner, RegexVariableMapper>, key: &Key) -> bool {
        workflow.get_state_controls().iter().any(|control| control.get_key() == *key) 
        || self.body_view_model.needs_update(&state.body_state, workflow, key)
        || self.legend_view_model.needs_update(&(), workflow, key)
        || self.titlebar_view_model.needs_update(&(), workflow, key)
    }
}

pub (crate) struct MainState {
    body_state: BodyState,
}

impl MainState {
    pub fn new() -> Self {
        let body_state = BodyState::new();
        Self {
            body_state,
        }
    }
}
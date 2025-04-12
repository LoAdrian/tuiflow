use std::{cell::{Ref, RefCell}, rc::Rc};

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::{StatefulWidgetRef, WidgetRef}};


use crate::{input::{ObservableKeyboard, Observer}, model::{control::Key, TerminalFlow}};

use super::{body::{BodyState, BodyWidget}, controls_widget::{ControlsWidget, WIDGET_PADDING_VERTICAL}, titlebar_widget::TitleBarWidget};

pub(crate) struct MainWidget<T: TerminalFlow> {
    workflow: T,
    title_bar: TitleBarWidget,
    body: Rc<RefCell<BodyWidget>>,
    legend_footer: ControlsWidget,
    body_state: Rc<RefCell<BodyState>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: TerminalFlow> MainWidget<T> {
    pub fn new(workflow: T, keyboard_observable: &mut ObservableKeyboard, title_bar: TitleBarWidget, body: Rc<RefCell<BodyWidget>>, legend_footer: ControlsWidget) -> Self {
        Self {
            workflow,
            title_bar,
            body,
            legend_footer,
            body_state: BodyState::new(keyboard_observable, Key::Up, Key::Down),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: TerminalFlow> WidgetRef for MainWidget<T> {
    type State = Rc<RefCell<MainViewModel<T>>>;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, view_model: &mut Self::State) {
        let legened_row_count =
            (self.legend_footer.get_legend_size() as f32 / 3.0).ceil() as u16 + WIDGET_PADDING_VERTICAL;
        let layout = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(legened_row_count),
        ]);
        let [title_bar, body, legend] = layout.areas(area);
        self.title_bar.render_ref(title_bar, buf);
        self.body.borrow().render_ref(body, buf, &mut self.body_state.clone());
        self.legend_footer.render_ref(legend, buf)
    }
    
}

impl<'a, T: TerminalFlow> Observer for MainWidget<T> {
    fn update(&mut self, key: Key) {
        self.workflow.run_control(self.body_state.borrow().get_selected_line_index(), &key);
    }
}
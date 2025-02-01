use std::{cell::RefCell, rc::Rc};

use buildable_macro::buildable;
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::WidgetRef};


use super::{body::BodyWidget, legend_widget::{LegendWidget, WIDGET_PADDING_VERTICAL}, titlebar_widget::TitleBarWidget};

pub struct MainWidget {
    title_bar: TitleBarWidget,
    body: Rc<RefCell<BodyWidget>>,
    legend_footer: LegendWidget,
}

#[buildable]
impl MainWidget {
    pub fn new(title_bar: TitleBarWidget, body: Rc<RefCell<BodyWidget>>, legend_footer: LegendWidget) -> Self {
        Self {
            title_bar,
            body,
            legend_footer,
        }
    }
}

impl WidgetRef for MainWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let legened_row_count =
            (self.legend_footer.get_legend_size() as f32 / 3.0).ceil() as u16 + WIDGET_PADDING_VERTICAL;
        let layout = Layout::vertical([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(legened_row_count),
        ]);
        let [title_bar, body, legend] = layout.areas(area);
        self.title_bar.render_ref(title_bar, buf);
        self.body.borrow().render_ref(body, buf);
        self.legend_footer.render_ref(legend, buf)
    }

}
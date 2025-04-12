use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidgetRef, WidgetRef},
};

use crate::State;

use super::key_control_view_model::KeyControlViewModel;

#[derive(Clone)]
pub struct ControlsWidget {
    main_block: Block<'static>,
}

impl ControlsWidget {
    pub fn new(entries: Vec<KeyControlViewModel>) -> Self {

        Self {
            main_block: Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            entries: entries
                .iter()
                .map(|entry| Paragraph::new(String::from(entry)))
                .collect::<Vec<Paragraph<'_>>>()
        }
    }
    pub fn get_legend_size(&self) -> usize {
        self.entries.len()
    }
}

pub const WIDGET_PADDING_VERTICAL: u16 = 2;
impl StatefulWidgetRef for ControlsWidget {
    type State = ControlsWidgetState;
    // TODO: This is still very dynamic. Either i should make it more static by doing more inside the constructor or actually use the dynamicness and calculate the layout based on screen-size
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let legend_size = self.get_legend_size();
        let column_count = if legend_size < 3 {
            legend_size as u32
        } else {
            3
        };
        let horizontal_constraints = (0..column_count)
            .map(|_| Constraint::Ratio(1, column_count))
            .collect::<Vec<_>>();
        let horizontal_layout = Layout::horizontal(horizontal_constraints);
        let row_count = (legend_size as f32 / 3.0).ceil() as u32;
        let vertical_constraints = (0..row_count)
            .map(|_| Constraint::Ratio(1, row_count))
            .collect::<Vec<_>>();
        let vertical_layout = Layout::vertical(vertical_constraints);

        let rows = vertical_layout.split(self.main_block.inner(area));
        let fields = rows.iter().flat_map(|row| {
            horizontal_layout
                .split(*row)
                .iter()
                .copied()
                .collect::<Vec<_>>()
        });
        self.main_block.render_ref(area, buf);

        state.entries
            .iter()
            .zip(fields)
            .for_each(|(entry, field): (&Paragraph<'_>, Rect)| entry.render_ref(field, buf));
    }
}

struct ControlsWidgetState<'a> {
    entries: Vec<Paragraph<'a>>,
}

impl<'a> ControlsWidgetState<'a> {
    pub fn new(entries: Vec<KeyControlViewModel>) -> Self {
        Self {
            entries: entries
                .iter()
                .map(|entry| Paragraph::new(String::from(entry)))
                .collect::<Vec<Paragraph<'a>>>(),
        }
    }

    pub fn update(&mut self, entries: Vec<KeyControlViewModel>) {
        self.entries = entries
            .iter()
            .map(|entry| Paragraph::new(String::from(entry)))
            .collect::<Vec<Paragraph<'a>>>();
    }
}
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, WidgetRef},
};

use super::key_control_view_model::KeyControlViewModel;

pub struct LegendWidget {
    main_block: Block<'static>,
    entries: Vec<Paragraph<'static>>,
}

impl LegendWidget {
    pub fn new(entries: Vec<KeyControlViewModel>) -> Self {
        Self {
            main_block: Block::new()
                .style(Style::default().bg(Color::LightGreen).fg(Color::Blue))
                .borders(Borders::ALL),
            entries: entries
                .iter()
                .map(|entry| Paragraph::new(String::from(entry)).bg(Color::Gray))
                .collect::<Vec<Paragraph<'_>>>(),
        }
    }
}

pub const WIDGET_PADDING_VERTICAL: u16 = 2;
impl WidgetRef for LegendWidget {
    // TODO: This is still very dynamic. Either i should make it more static by doing more inside the constructor or actually use the dynamicness and calculate the layout based on screen-size
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let column_count = if self.entries.len() < 3 {
            self.entries.len() as u32
        } else {
            3
        };
        let horizontal_constraints = (0..column_count)
            .map(|_| Constraint::Ratio(1, column_count))
            .collect::<Vec<_>>();
        let horizontal_layout = Layout::horizontal(horizontal_constraints);
        let row_count = (self.entries.len() as f32 / 3.0).ceil() as u32;
        let vertical_constraints = (0..row_count)
            .map(|_| Constraint::Ratio(1, row_count))
            .collect::<Vec<_>>();
        let vertical_layout = Layout::vertical(vertical_constraints);

        let block = Block::new()
            .style(Style::default().bg(Color::LightGreen).fg(Color::Blue))
            .borders(Borders::ALL);
        let block_content = block.inner(area);

        let rows = vertical_layout.split(block_content);
        let fields = rows.iter().flat_map(|row| {
            horizontal_layout
                .split(*row)
                .iter()
                .copied()
                .collect::<Vec<_>>()
        });
        block.render_ref(area, buf);

        self.entries
            .iter()
            .zip(fields)
            .for_each(|(entry, field): (&Paragraph<'_>, Rect)| entry.render_ref(field, buf));
    }
}

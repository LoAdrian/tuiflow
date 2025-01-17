use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph, Widget, WidgetRef}};

pub struct TitleBarWidget {
    app_title: Paragraph<'static>,
    state_title: Paragraph<'static>,
}

impl TitleBarWidget {
    pub fn new(app_title: impl Into<String>, state_title: impl Into<String>) -> Self {
        Self { 
            app_title: Paragraph::new(app_title.into())
                .centered(), 
            state_title: Paragraph::new(state_title.into())
                .centered()
                .bold()
                .underlined(), 
        }
    }
}

impl WidgetRef for TitleBarWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .style(Style::default().bg(Color::LightBlue).fg(Color::Blue))
            .borders(Borders::ALL);
        let block_content = block.inner(area);
        block.render(area, buf);

        let layout = Layout::vertical([
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 2),
        ]);
        let [app_title_area, state_title_area] = layout.areas(block_content);

        self.app_title.render_ref(app_title_area, buf);
        self.state_title.render_ref(state_title_area, buf);
    }
}
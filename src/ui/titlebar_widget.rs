use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, text::Text, widgets::{Block, Borders, Paragraph, Widget, WidgetRef}};

#[derive(Clone)]
pub struct TitleBarWidget {
    app_title: Paragraph<'static>,
    state_title: Paragraph<'static>,
    logo: Paragraph<'static>,
}

impl TitleBarWidget {
    pub fn new(app_title: String, state_title: String) -> Self {
        let logo_str = 
" 
 _____  __  __  __  ____  __     ______  __    __
/_  _/ / / / / / / / __/ / /    / __  / / /_  / /
 / /  / /_/ / / / / __/ / /__  / /_/ / / // |/ / 
/_/  /_____/ /_/ /_/   /____/ /_____/ /___/|__/  ".trim_start();

        let logo = Paragraph::new(logo_str)
            .style(Style::default()
                .fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Right);

        Self { 
            app_title: Paragraph::new(app_title)
                .bold(),
            state_title: Paragraph::new(state_title),
            logo
        }
    }
}

impl WidgetRef for TitleBarWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {

        let horizontal_layout = Layout::horizontal([
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 2),
        ]);

        let vertical_layout = Layout::vertical([
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 2),
        ]);
        let [app_area, logo_area] = horizontal_layout.areas(area);

        /*let block = Block::new()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);
        let block_content = block.inner(area);
        block.render(app_area, buf);*/

        let [app_title_area, state_title_area] = vertical_layout.areas(app_area);

        self.app_title.render_ref(app_title_area, buf);
        self.state_title.render_ref(state_title_area, buf);
        self.logo.render_ref(logo_area, buf);

    }
}
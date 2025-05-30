use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, widgets::{Paragraph, WidgetRef}};
use tuiflow_model::control::Key;
use tuiflow_model::TerminalFlow;
use crate::io::sh_command_runner::ShCommandRunner;
use crate::{io::InputUpdatedViewModel, RegexVariableMapper, Workflow};

#[derive(Clone)]
pub struct TitleBarWidget<'a> {
    app_title: Paragraph<'a>,
    state_title: Paragraph<'a>,
    logo: Paragraph<'a>,
}
const LOGO_STR: &str = 
" _____  __  __  __  ____  __     ______  __    __
/_  _/ / / / / / / / __/ / /    / __  / / /_  / /
 / /  / /_/ / / / / __/ / /__  / /_/ / / // |/ / 
/_/  /_____/ /_/ /_/   /____/ /_____/ /___/|__/  ";

impl<'a> TitleBarWidget<'a> {
    pub fn new(view_model: &'a TitleBarViewModel) -> Self {
        let logo = Paragraph::new(LOGO_STR)
            .style(Style::default()
                .fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Right);

        Self { 
            app_title: Paragraph::new(view_model.app_title.as_str())
                .bold(),
            state_title: Paragraph::new(view_model.state_title.as_str()),
            logo
        }
    }
}

impl<'a> WidgetRef for TitleBarWidget<'a> {
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

        let [app_title_area, state_title_area] = vertical_layout.areas(app_area);

        self.app_title.render_ref(app_title_area, buf);
        self.state_title.render_ref(state_title_area, buf);
        self.logo.render_ref(logo_area, buf);

    }
}

pub(crate) struct TitleBarViewModel {
    app_title: String,
    state_title: String,
}

impl <'a> TitleBarViewModel {
    pub fn new(workflow: &Workflow<ShCommandRunner, RegexVariableMapper>) -> Self {
        Self {
            app_title: workflow.get_app_title().to_string(),
            state_title: workflow.get_state_title().to_string(),
        }
    }
    
}

impl InputUpdatedViewModel for TitleBarViewModel {
    type ViewState = ();

    fn needs_update(&self, _: &Self::ViewState, _: & impl TerminalFlow, _: &Key) -> bool {
        false
    }

    fn update(&mut self, _: &mut Self::ViewState, workflow: &mut impl TerminalFlow, _: &Key) {
        let current_state_title = workflow.get_state_title();
        if self.state_title != *current_state_title {
            self.state_title = current_state_title.to_string()
        }
    }
}
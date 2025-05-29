use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, WidgetRef},
};

use crate::{
    input::InputUpdatedViewModel,
    model::{Control, TerminalFlow},
    workflow::ShCommandRunner,
    RegexVariableMapper, Workflow,
};

use super::key_control_view_model::KeyControlViewModel;

#[derive(Clone)]
pub(crate) struct ControlsWidget<'a> {
    main_block: Block<'a>,
    entries: Vec<Paragraph<'a>>,
}

impl<'a> ControlsWidget<'a> {
    pub fn new(view_model: &ControlsViewModel) -> Self {
        Self {
            main_block: Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            entries: view_model
                .entries
                .iter()
                .map(|entry| Paragraph::new(String::from(entry)))
                .collect::<Vec<Paragraph<'a>>>(),
        }
    }
    pub fn get_legend_size(&self) -> usize {
        self.entries.len()
    }
}

pub(crate) const WIDGET_PADDING_VERTICAL: u16 = 2;
impl<'a> WidgetRef for ControlsWidget<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.main_block.render_ref(area, buf);
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
        self.entries
            .iter()
            .zip(fields)
            .for_each(|(entry, field): (&Paragraph<'_>, Rect)| entry.render_ref(field, buf));
    }
}

pub(crate) struct ControlsViewModel {
    entries: Vec<KeyControlViewModel>,
    selection_up: Control,
    selection_down: Control,
}

impl ControlsViewModel {
    pub fn new(
        workflow: &Workflow<ShCommandRunner, RegexVariableMapper>,
        selection_up: Control,
        selection_down: Control,
    ) -> Self {
        let mut entries: Vec<KeyControlViewModel> = workflow
            .get_state_controls()
            .into_iter()
            .map(|control| KeyControlViewModel::new(control))
            .collect();

        entries.push(KeyControlViewModel::new(selection_up.clone()));
        entries.push(KeyControlViewModel::new(selection_down.clone()));

        Self { entries, selection_up, selection_down}
    }
}

impl InputUpdatedViewModel for ControlsViewModel {
    type ViewState = ();

    fn needs_update(
        &self,
        _: &Self::ViewState,
        _: &impl TerminalFlow,
        _: &crate::model::control::Key,
    ) -> bool {
        false
    }

    fn update(
        &mut self,
        _: &mut Self::ViewState,
        workflow: &mut impl TerminalFlow,
        _: &crate::model::control::Key,
    ) {
        self.entries = workflow
            .get_state_controls()
            .into_iter()
            .map(|control| KeyControlViewModel::new(control))
            .collect();

        self.entries.push(KeyControlViewModel::new(self.selection_up.clone()));
        self.entries.push(KeyControlViewModel::new(self.selection_down.clone()));
    }
}

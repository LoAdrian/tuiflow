use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, StatefulWidgetRef, WidgetRef,
    },
};

use crate::{
    input::InputUpdatedViewModel,
    model::{control::Key, Display, Line, TerminalFlow},
    workflow::ShCommandRunner,
    RegexVariableMapper, Workflow,
};

// TODO: find a better solution than RefCell for everything mutable
pub(crate) struct BodyWidget<'a> {
    main_block: Block<'static>,
    list: List<'a>,
}

impl<'a> BodyWidget<'a> {
    pub fn new(view_model: &'a BodyViewModel) -> Self {
        let list_items = view_model
            .get_list_items()
            .into_iter()
            .map(|item: &str| ListItem::<'a>::new(ratatui::text::Line::<'a>::raw(item)))
            .collect::<Vec<_>>();

        Self {
            main_block: Block::new()
                .style(Style::default())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            list: List::new(list_items).highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .bg(ratatui::style::Color::Cyan),
            ),
        }
    }
}

impl<'a> StatefulWidgetRef for BodyWidget<'a> {
    type State = BodyState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.main_block.render_ref(area, buf);
        let block_content_area = self.main_block.inner(area);
        StatefulWidgetRef::render_ref(&self.list, block_content_area, buf, &mut state.list_state)
    }
}

pub(crate) struct BodyViewModel {
    display: Display,
    selection_up: Key,
    selection_down: Key,
}

impl BodyViewModel {
    pub fn new(display: Display, selection_up: Key, selection_down: Key) -> Self {
        //TODO: at some point use references for display
        Self {
            display,
            selection_down,
            selection_up,
        }
    }

    pub fn get_list_items<'a>(&'a self) -> Vec<&'a str> {
        self.display
            .lines
            .iter()
            .map(|line| line.0.as_str())
            .collect::<Vec<&'a str>>()
    }
}

impl InputUpdatedViewModel for BodyViewModel {
    type ViewState = BodyState;
    fn update(
        &mut self,
        state: &mut Self::ViewState,
        workflow: &mut Workflow<ShCommandRunner, RegexVariableMapper>,
        key: &Key,
    ) {
        if *key == self.selection_down {
            state.move_selection_down();
        } else if *key == self.selection_up {
            state.move_selection_up();
        }

        self.display = workflow.get_display().clone()
    }

    fn needs_update(
        &self,
        _: &Self::ViewState,
        _: &Workflow<ShCommandRunner, RegexVariableMapper>,
        key: &Key,
    ) -> bool {
        *key == self.selection_down || *key == self.selection_up
    }
}

pub(crate) struct BodyState {
    list_state: ListState,
}

impl BodyState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();
        Self { list_state }
    }
    pub fn move_selection_down(&mut self) {
        let next_selection = self.list_state.selected().and_then(|i| Some(i + 1));
        self.list_state.select(next_selection);
    }

    pub fn move_selection_up(&mut self) {
        let next_selection =
            self.list_state
                .selected()
                .and_then(|i| if i > 0 { Some(i - 1) } else { Some(i) });
        self.list_state.select(next_selection);
    }

    pub fn get_selected_line_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }
}

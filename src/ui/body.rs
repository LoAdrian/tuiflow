use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use ratatui::{buffer::Buffer, layout::Rect, style::{Modifier, Style}, text::{Line, Text}, widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, StatefulWidgetRef, WidgetRef}};

use crate::{input::{ObservableKeyboard, Observer}, model::control::Key};

// TODO: find a better solution than RefCell for everything mutable
pub(crate) struct BodyWidget {
    main_block: Block<'static>,
    list: List<'static>,
    x: String
}

impl BodyWidget {
    pub fn new(list_content: Vec<String>) -> Rc<RefCell<Self>> {
        let list_items = list_content.iter().map(|item| {
            ListItem::new(Line::raw(item.to_string()))
        }).collect::<Vec<_>>();

        let self_ref = Rc::new(RefCell::new(Self {
            main_block: Block::new()
                .style(Style::default())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            list: List::new(list_items)
                .highlight_style(
                    Style::new()
                        .add_modifier(Modifier::BOLD)
                        .bg(ratatui::style::Color::Cyan)),
            x: "asdf".to_string()
        }));
        
        self_ref
    }
}

impl StatefulWidgetRef for BodyWidget {
    type State = Rc<RefCell<BodyState>>;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.main_block.render_ref(area, buf);
        let block_content_area = self.main_block.inner(area);
        StatefulWidgetRef::render_ref(&self.list, block_content_area, buf, &mut state.borrow_mut().list_state)
    }
}

pub(crate) struct BodyViewModel<'a> {
    view_state: BodyState,
}

impl<'a> BodyViewModel<'a> {
    pub fn new() -> Self {
        Self {
            view_state: BodyState::new(observable_keyboard, select_up, select_down)
        }
    }

    pub fn get_view_state(&mut self) -> &mut BodyState {
        &mut self.view_state
    }

    pub fn get_list_items<T>() -> T where T : IntoIterator, T::Item : Into<ListItem<'a>> {

    }
}

pub(crate) struct BodyState {
    list_state: ListState,
    select_up: Key,
    select_down: Key,
}

impl BodyState {
    pub fn new(observable_keyboard: &mut ObservableKeyboard, select_up: Key, select_down: Key) -> Rc<RefCell<Self>> {

        let self_ref = Rc::new(RefCell::new(Self {
            list_state: ListState::default(),
            select_up,
            select_down,
        }));
        self_ref.borrow_mut().list_state.select_first();
        
        observable_keyboard.register(select_down, self_ref.clone());
        observable_keyboard.register(select_up, self_ref.clone());
        self_ref
    }
    pub fn move_selection_down(&mut self) {
        let next_selection = self.list_state.selected().and_then(|i| {Some(i + 1)});
        self.list_state.select(next_selection);
    }
    
    pub fn move_selection_up(&mut self) {
        let next_selection = self.list_state.selected().and_then(|i| { if i > 0 {Some(i - 1)} else {Some(i)} });
        self.list_state.select(next_selection);
    }

    pub fn get_selected_line_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }
}

impl Observer for BodyState {
    fn update(&mut self, key: Key) {
        if key == self.select_down {
            self.move_selection_down();
        } else if key == self.select_up {
            self.move_selection_up();
        }
    }
}

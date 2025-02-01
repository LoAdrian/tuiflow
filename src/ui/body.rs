use std::{cell::RefCell, rc::Rc};

use buildable_macro::buildable;
use ratatui::{buffer::Buffer, layout::Rect, style::{Modifier, Style}, text::Line, widgets::{Block, Borders, List, ListItem, ListState, StatefulWidgetRef, WidgetRef}};

use crate::{input::{ObservableKeyboard, Observer}, model::control::Key};

// TODO: find a better solution than RefCell for everything mutable
#[buildable]
pub struct BodyWidget {
    main_block: Block<'static>,
    list: List<'static>,
    list_state: RefCell<ListState>,
    select_up: Key,
    select_down: Key,
}

impl BodyWidget {
    pub fn new(observable_keyboard: &mut ObservableKeyboard, select_up: Key, select_down: Key, list_content: Vec<String>) -> Rc<RefCell<Self>> {
        let x = BodyWidgetBuilder::new();
        let list_items = list_content.iter().map(|item| {
            ListItem::new(Line::raw(item.to_string()))
        }).collect::<Vec<_>>();

        let self_ref = Rc::new(RefCell::new(Self {
            main_block: Block::new()
                .style(Style::default())
                .borders(Borders::ALL),
            list: List::new(list_items).highlight_style(Style::new().add_modifier(Modifier::BOLD)),
            list_state: RefCell::new(ListState::default()),
            select_down,
            select_up,
        }));
        
        self_ref.borrow_mut().list_state.borrow_mut().select_first();
        
        observable_keyboard.register(select_down, self_ref.clone());
        observable_keyboard.register(select_up, self_ref.clone());
        self_ref
    }

    pub fn move_selection_down(&mut self) {
        let next_selection = self.list_state.borrow_mut().selected().and_then(|i| {Some(i + 1)});
        self.list_state.borrow_mut().select(next_selection);
    }
    
    pub fn move_selection_up(&mut self) {
        let next_selection = self.list_state.borrow_mut().selected().and_then(|i| { if i > 0 {Some(i - 1)} else {Some(i)} });
        self.list_state.borrow_mut().select(next_selection);
    }
}

impl WidgetRef for BodyWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.main_block.render_ref(area, buf);
        let block_content_area = self.main_block.inner(area);
        StatefulWidgetRef::render_ref(&self.list, block_content_area, buf, &mut self.list_state.borrow_mut())
    }
}

impl Observer for BodyWidget {
    fn update(&mut self, key: Key) {
        if key == self.select_down {
            self.move_selection_down();
        } else if key == self.select_up {
            self.move_selection_up();
        }
    }
}
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::{Block, Borders, List, ListItem, ListState, StatefulWidgetRef, WidgetRef}};

use crate::input::Observer;

pub struct BodyWidget {
    main_block: Block<'static>,
}

impl BodyWidget {
    pub fn new() -> Self {
        Self {
            main_block: Block::new()
                .style(Style::default())
                .borders(Borders::ALL),
        }   
    }
}

impl StatefulWidgetRef for BodyWidget {
    type State = String;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.main_block.render_ref(area, buf);
        let block_content_area = self.main_block.inner(area);
        let list_content = vec!["Hello", "World", "Foo", "Bar"];
        let list_items = list_content.iter().map(|item| {
            ListItem::new(Line::raw(item.to_string()))
        });
        let list = List::new(list_items);
        let mut list_state = ListState::default();
        StatefulWidgetRef::render_ref(&list, block_content_area, buf, &mut list_state)
    }
}

pub struct BodyWidgetViewModel {
    list_state: ListState,
}

impl Observer for BodyWidgetViewModel {
    fn update(&mut self, key: char) {
    }
}
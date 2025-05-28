
#[derive(Clone, PartialEq)]
pub struct Display { // Make this iterable
    pub lines: Vec<Line>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            lines: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Line(pub String); // Make this Into<ListItem>
#[derive(Clone)]
pub struct Display {
    pub lines: Vec<Line>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            lines: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Line(pub String);

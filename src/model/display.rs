#[derive(Clone)]
pub struct Display {
    pub lines: Vec<Line>,
    pub errors: Vec<String>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            errors: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Line(pub String);

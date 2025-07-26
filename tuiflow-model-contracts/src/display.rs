#[derive(Debug, Clone, PartialEq)]
pub struct Display { // Make this iterable
    pub lines: Vec<Line>,
    pub errors: Vec<DisplayError<>>
}

impl Default for Display {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            errors: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line(pub String);

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Line {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayError(pub String);
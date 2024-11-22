#[derive(Clone)]
pub struct Display {
    pub lines: Vec<Line>,
    pub error: String,
    pub info: String
}

#[derive(Clone)]
pub struct Line {
    content: String
}
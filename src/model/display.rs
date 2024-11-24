#[derive(Clone)]
pub struct Display {
    pub lines: Vec<Line>,
    pub error: Option<String>,
    pub info: Option<String>
}

impl Default for Display {
    fn default() -> Self {
        Self { 
            lines: Default::default(), 
            error: Default::default(),
            info: Default::default() 
        }
    }
}

#[derive(Clone)]
pub struct Line(pub String);
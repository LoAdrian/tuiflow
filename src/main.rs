use core::fmt;
use std::error::Error;

use crossterm::event::{self, Event};
use ratatui::{text::Text, widgets::Paragraph, Frame};
use regex::Regex;

mod model;

fn main() {

}

fn draw(frame: &mut Frame) {
    let text = Paragraph::new("Hello, world!");
    frame.render_widget(text, frame.area());
}

enum FLowState {
    List(ListState),
    Raw(RawState)
}

struct ListState {
    delimiter: String,
    line_filter: String,
    line_display: String,
    transitions: Vec<Transition>
}

struct RawState {
    filter: String,
    display: String,
    transitions: Vec<Transition>
}

struct Transition {
    control: Control,
    next_state: FLowState,
    filter: String, // regex extraction from selection
    command: String, //command to execute
}

struct Control {
    name: String,
    key: char // probably should not be char
}

struct Flow {
    // Initial State with initial transition leading to the state, or maybe just an initial command?
    // Basic settings
    // Factory method to create the flow
    // Maybe / Probably a title
}
use std::{cell::RefCell, rc::Rc};

use app::App;
use eyre::Result;
use model::{control::Key, state::{builder::StateBuilder, State}, transition::builder::TransitionBuilder, variable_mapping::{RegexVariableMapper, VariableMapper}, workflow::{self, CommandRunner, ShCommandRunner, Workflow}, Control, Line};

mod model;
mod app;
mod ui;
mod input;

pub fn main() -> Result<()> {
    let terminal = ratatui::init();

    let command_runner = ShCommandRunner;

    let initial_state = StateBuilder::new()
        .with_command_output_to_display_mapper(RegexVariableMapper::new("(?<path>.+)", "<path>").expect(""))
        .with_display_name("Show files".to_string())
        .with_command_runner(command_runner.clone())
        .build();
    let initial_state_ref = Rc::new(RefCell::new(initial_state));

    let move_into = TransitionBuilder::new()
        .with_control(Control::new("move into", Key::Char('l')))
        .with_selected_display_to_command(RegexVariableMapper::new("(?<x>.*)", "ls -d -1 \"<x>/\"**").expect("failed to create mapper"))
        .with_next_state(initial_state_ref.clone())
        .build();

    let move_back = TransitionBuilder::new()
        .with_control(Control::new("move back", Key::Char('h')))
        .with_selected_display_to_command(RegexVariableMapper::new("(?<x>.*)\\/.*\\/.*", "ls -d -1 \"<x>/\"**").expect("failed to create mapper"))
        .with_next_state(initial_state_ref.clone())
        .build();

    initial_state_ref.borrow_mut().add_transition(Key::Char('l'), move_into);
    initial_state_ref.borrow_mut().add_transition(Key::Char('h'), move_back);

    let initial_transition = TransitionBuilder::new()
        .with_control(Control::new("move into", Key::Char('l')))
        .with_selected_display_to_command(RegexVariableMapper::new(".*", "ls -d -1 \"$PWD/\"**").expect("failed to create mapper"))
        .with_next_state(initial_state_ref)
        .build();
    let initializer_state = State::new("INIT", RegexVariableMapper::identity(), vec![initial_transition], command_runner.clone());

    let workflow = Workflow::<ShCommandRunner, RegexVariableMapper>::new(initializer_state, Line{0: "DOES NOT CARE".to_string() }, "file explorer".to_string());

    App::new(Control::new("Quit", Key::Char('q'))).run(terminal, workflow);
    ratatui::restore();
    Ok(())
}

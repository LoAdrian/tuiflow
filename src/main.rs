use std::{cell::RefCell, env, rc::Rc};
use std::fs::File;
use app::App;
use eyre::Result;
use model::{control::Key, state::{builder::StateBuilder, State}, transition::builder::TransitionBuilder, variable_mapping::{RegexVariableMapper, VariableMapper}, workflow::{self, CommandRunner, ShCommandRunner, Workflow}, Control, Line};
use crate::app::configuration::AppConfiguration;

mod model;
mod app;
mod ui;
mod input;

pub fn main() -> Result<()> {
    let tuiflow_config_path = env::args().nth(1).unwrap(); //TODO: handle propertly
    
    let terminal = ratatui::init();
    let config: AppConfiguration = serde_yaml::from_reader(File::open(&tuiflow_config_path)?)?;

    App::new(config).run(terminal);
    ratatui::restore();
    Ok(())
}

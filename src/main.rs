use crate::app::configuration::AppConfiguration;
use app::App;
use eyre::Result;
use model::{variable_mapping::{RegexVariableMapper}, workflow::{self, Workflow}};
use std::fs::File;
use std::env;

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

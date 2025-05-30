use crate::app::configuration::AppConfiguration;
use app::App;
use eyre::OptionExt;
use tuiflow_model::{
    variable_mapping::RegexVariableMapper,
    workflow::Workflow,
};
use std::env;
use std::fs::File;

mod app;
mod io;
mod ui;

pub fn main() -> eyre::Result<()> {
    let tuiflow_config_path = read_config_path_or_print_err()?;
    let config = read_config_or_print_err(tuiflow_config_path)?;

    let terminal = ratatui::init();
    App::new(config)?.run(terminal)?;
    ratatui::restore();
    Ok(())
}

fn read_config_path_or_print_err() -> eyre::Result<String> {
    let config_path = env::args()
        .nth(1)
        .ok_or_eyre("Invalid arguments. Usage: tuiflow <path_to_config>")?;
    Ok(config_path)
}

fn read_config_or_print_err(config_path: String) -> eyre::Result<AppConfiguration> {
    let file = File::open(&config_path).map_err(|e| {
        println!("Error: Could not open configuration file: {}. Check if the file exists and is accessible.", config_path);
        e
    })?;

    let config: AppConfiguration = serde_yaml::from_reader(file).map_err(|e| {
        println!("Error: Could not parse configuration file.");
        e
    })?;

    Ok(config)
}

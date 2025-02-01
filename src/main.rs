use app::App;
use eyre::Result;

mod model;
mod app;
mod ui;
mod input;

pub fn main() -> Result<()> {
    let terminal = ratatui::init();
    App::new().run(terminal)?;
    ratatui::restore();
    Ok(())
}
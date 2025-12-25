mod app;
mod input;
mod state;

mod components;
mod features;

use color_eyre::eyre;

use app::App;

pub fn start() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::default();

    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

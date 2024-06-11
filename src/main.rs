use color_eyre::Result;

mod app;
mod errors;
mod tui;

type App = app::App;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

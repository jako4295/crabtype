use color_eyre::Result;

mod docs_example;
// mod app;
// mod errors;
// mod tui;

type App = docs_example::app::App;

fn main() -> Result<()> {
    docs_example::errors::install_hooks()?;
    let mut terminal = docs_example::tui::init()?;
    App::default().run(&mut terminal)?;
    docs_example::tui::restore()?;
    Ok(())
}

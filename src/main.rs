mod menu;
mod settings;
mod tui;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = tui::tui_tools::init()?;
    tui::pages::App::default().run(&mut terminal)?;
    tui::tui_tools::restore()
}

// mod docs_example;
// use color_eyre::{eyre::Ok, Result};
// fn run_docs_example() -> Result<()> {
//     docs_example::errors::install_hooks()?;
//     let mut terminal = docs_example::tui::init()?;
//     docs_example::app::App::default().run(&mut terminal)?;
//     docs_example::tui::restore()?;s
//     Ok(())
// }

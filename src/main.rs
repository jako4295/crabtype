mod mvp;

fn main() {
    let _ = mvp::tui::init();
    let res = mvp::tui::restore();
    match res {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }
}

// mod docs_example;
// use color_eyre::{eyre::Ok, Result};
// fn run_docs_example() -> Result<()> {
//     docs_example::errors::install_hooks()?;
//     let mut terminal = docs_example::tui::init()?;
//     docs_example::app::App::default().run(&mut terminal)?;
//     docs_example::tui::restore()?;
//     Ok(())
// }

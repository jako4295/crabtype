mod mvp;

fn main() {
    let terminal_result = mvp::tui::init();
    match terminal_result {
        Ok(terminal) => mvp::app::App::default().run(&mut terminal),
        Err(err) => println!("{:?}", err),
    }
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
//     docs_example::tui::restore()?;s
//     Ok(())
// }

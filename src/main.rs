use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    loop {
        draw_tui(&mut terminal);
        match handle_event() {
            EventKey::Quit => break,
            EventKey::Nothing => {}
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

enum EventKey {
    Quit,
    Nothing,
}

fn handle_event() -> EventKey {
    let poll_result = event::poll(std::time::Duration::from_millis(16));
    match poll_result {
        Ok(true) => {
            let event_result = event::read();
            match event_result {
                Ok(event::Event::Key(key)) => {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        return EventKey::Quit;
                    }
                }
                _ => return EventKey::Nothing,
            }
        }
        _ => return EventKey::Nothing,
    }
    return EventKey::Nothing;
}

fn draw_tui(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    let result = terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );
    });
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

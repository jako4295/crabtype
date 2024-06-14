use crate::mvp::tui;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Debug, Default)]
pub struct App {
    current_char: char,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(code) => self.display_pressed_char(code),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn display_pressed_char(&mut self, character: char) {
        self.current_char = character;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .fg(Color::Blue)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .border_set(border::THICK);

        let crabtype: String = {
            "
                                                                       
                                                                       
       ████████                 ████████████████                   
      ████████                       ██████████                    
     ████    ███████████████████████     █████████████ 
    ████     ██ █ ██ █ ██ ████████ ████      
   ████     ██████ ██████████████████ ██████████████   
  ██████████   ██ █ ██ ██████  ██    ███        
  ██████████ ██████ ██████████████  ██   █████████    
                                                                       
                                                                       
"
            .to_string()
        };

        let menu_options: String = {
            "
    [b]egin
    [s]ettings
    [q]uit
        "
            .to_string()
        };
        let comb_str = crabtype + &menu_options;

        Paragraph::new(comb_str).block(block).render(area, buf);
    }
}

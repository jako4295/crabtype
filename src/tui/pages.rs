use core::panic;
use std::io;

use crate::tui::tui_tools;
use crate::{menu::menu_page, settings::settings_page};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

#[derive(Debug, Default)]
pub struct App<'a> {
    current_char: char,
    exit: bool,
    // app state:
    state: &'a str,
}

impl<'a> App<'a> {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui_tools::Tui) -> io::Result<()> {
        self.state = "menu";
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
        // options for the different pages:
        // Menu:
        if self.state == "menu" {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('s') => {
                    self.state = "settings";
                }
                _ => {}
            }

        // Settings:
        } else if self.state == "settings" {
            match key_event.code {
                KeyCode::Esc => {
                    self.state = "menu";
                }
                _ => {}
            }

        // Game:
        } else if self.state == "game" {
            match key_event.code {
                KeyCode::Esc => {
                    self.state = "menu";
                }
                KeyCode::Char(code) => self.display_pressed_char(code),
                _ => {}
            }
        } else {
            panic!("Unexpected state");
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn display_pressed_char(&mut self, character: char) {
        self.current_char = character;
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            "menu" => menu_page::render(area, buf),
            "settings" => settings_page::render(area, buf),
            _ => {}
        }
    }
}

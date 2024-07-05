use core::panic;
use std::io;

use crate::tui::tui_tools;
use crate::{game::game_page, menu::menu_page, settings::settings_page};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

#[derive(Debug, Default)]
pub struct App<'a> {
    exit: bool,
    // app state:
    state: &'a str,
    gamestruct: game_page::GameLogic,
    settings_select: settings_page::SettingsStateList,
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
                KeyCode::Char('b') => {
                    self.state = "game";
                }
                _ => {}
            }

        // Settings:
        } else if self.state == "settings" {
            match key_event.code {
                KeyCode::Esc => {
                    self.state = "menu";
                }
                KeyCode::Char('j') | KeyCode::Down => self.settings_select.next(),
                KeyCode::Char('k') | KeyCode::Up => self.settings_select.previous(),
                KeyCode::Char('h') | KeyCode::Left => self.settings_select.unselect(),
                _ => {}
            }

        // Game:
        } else if self.state == "game" {
            self.gamestruct.get_time();
            match key_event.code {
                KeyCode::Esc => {
                    self.state = "menu";
                }
                KeyCode::Char(' ') => self.gamestruct.reset(),
                KeyCode::Char(code) => self.gamestruct.compare_pressed_char(code),
                _ => {}
            }
        } else {
            panic!("Unexpected state");
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            "menu" => menu_page::render(area, buf),
            "settings" => {
                let mut states = settings_page::SettingsStateList::default();
                states.render(area, buf);
            }
            "game" => {
                self.gamestruct.render(area, buf);
            }
            _ => {}
        }
    }
}

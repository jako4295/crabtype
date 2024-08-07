use core::panic;
use std::cell::RefCell;
use std::io;
use std::time::Duration;
use std::time::Instant;

use crate::tui::tui_tools;
use crate::{game::game_page, menu::menu_page, settings::settings_page};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

#[derive(Debug, Default)]
pub struct App<'a> {
    exit: bool,
    // app state:
    state: &'a str,
    gamestruct: RefCell<game_page::GameLogic>,
    settings_select: RefCell<settings_page::SettingsStateList>,
}

impl<'a> App<'a> {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui_tools::Tui) -> io::Result<()> {
        self.state = "menu";
        let tick_rate = Duration::from_millis(100); // Update every second
        let mut last_tick = Instant::now();

        while !self.exit {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                self.handle_events()?;
            }

            if last_tick.elapsed() >= tick_rate {
                self.gamestruct.borrow_mut().get_time(); // Update the game timer
                last_tick = Instant::now();
                terminal.draw(|frame| self.render_frame(frame))?;
            }
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
                    self.gamestruct = game_page::GameLogic::default().into();
                    self.gamestruct.borrow_mut().reset();
                }
                _ => {}
            }

        // Settings:
        } else if self.state == "settings" {
            match key_event.code {
                KeyCode::Esc => {
                    self.state = "menu";
                }
                KeyCode::Char('j') | KeyCode::Down => self.settings_select.borrow_mut().next(),
                KeyCode::Char('k') | KeyCode::Up => self.settings_select.borrow_mut().previous(),
                KeyCode::Char('h') | KeyCode::Left => {
                    self.settings_select.borrow_mut().edit_entry(false)
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    self.settings_select.borrow_mut().edit_entry(true)
                }
                _ => {}
            }

        // Game:
        } else if self.state == "game" {
            self.gamestruct.borrow_mut().get_time();
            match key_event.code {
                KeyCode::Esc => {
                    self.state = "menu";
                }
                KeyCode::Char(' ') => self.gamestruct.borrow_mut().reset(),
                KeyCode::Char(code) => self.gamestruct.borrow_mut().compare_pressed_char(code),
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
                self.settings_select.borrow_mut().render(area, buf);
            }
            "game" => {
                self.gamestruct.borrow_mut().render(area, buf);
            }
            _ => {}
        }
    }
}

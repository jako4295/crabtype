use crate::mvp::tui;
use crate::char_lib::load_chars;
use dict::{Dict, DictIface};
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Debug, Default)]
pub struct App {
    random_char: char,
    pressed_char: char,
    char_vec: Vec<char>,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let dict: Dict<bool> = self.get_dict();
        self.char_vec = load_chars::load_files_to_vec(dict);
        self.random_char = load_chars::chose_random(self.char_vec.to_owned());
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn get_dict (&self) -> Dict<bool>{
        let mut dict: Dict<bool> = Dict::<bool>::new();
        dict.add("letters".to_string(), true);
        dict.add("cap_letters".to_string(), false);
        dict.add("numbers".to_string(), false);
        dict
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
            KeyCode::Enter => self.exit(),
            KeyCode::Char(code) => self.compare_pressed_char(code),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn compare_pressed_char(&mut self, character: char) {
        // TODO compare pressed_char to random_char if correct
        // update the random char
        if self.random_char == character {
            self.random_char = load_chars::chose_random(self.char_vec.to_owned());
        }
        // self.pressed_char = character;
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" CrabType ".bold());
        // let instructions = Title::from(Line::from(vec![
        //     " Decrement ".into(),
        //     "<Left>".blue().bold(),
        //     " Increment ".into(),
        //     "<Right>".blue().bold(),
        //     " Quit ".into(),
        //     "<Q> ".blue().bold(),
        // ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            // .title(
            //     instructions
            //         .alignment(Alignment::Center)
            //         .position(Position::Bottom),
            // )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.random_char.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

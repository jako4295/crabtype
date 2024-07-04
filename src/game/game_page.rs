use crate::char_lib::load_chars;
use chrono::{DateTime, Duration, Local};
use dict::{Dict, DictIface};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

pub fn get_dict() -> Dict<bool> {
    let mut dict: Dict<bool> = Dict::<bool>::new();
    dict.add("letters".to_string(), true);
    dict.add("cap_letters".to_string(), false);
    dict.add("numbers".to_string(), false);
    dict
}

#[derive(Debug)]
pub struct GameLogic {
    pub time: Duration,
    pub start_time: DateTime<Local>,
    pub random_char: char,
    pub char_vec: Vec<char>,
    pub score: u32,
    pub play: bool,
    pub current_char: Option<char>,
}

impl Default for GameLogic {
    fn default() -> GameLogic {
        let dict: Dict<bool> = get_dict();
        let start_t = Local::now();
        let load_char = load_chars::load_files_to_vec(dict);

        GameLogic {
            time: Local::now().signed_duration_since(start_t),
            start_time: start_t,
            random_char: load_chars::chose_random(load_char.to_owned()),
            char_vec: load_char,
            score: 0,
            play: true,
            current_char: None,
        }
    }
}

impl GameLogic {
    pub fn get_time(&mut self) {
        let time_now = Local::now();
        self.time = time_now.signed_duration_since(self.start_time)
    }
    pub fn reset(&mut self) {
        self.start_time = Local::now();
        self.score = 0;
        self.play = true;
    }
    pub fn compare_pressed_char(&mut self, character: char) {
        if self.time >= Duration::seconds(30) {
            self.play = false
        }
        if self.random_char == character {
            self.random_char = load_chars::chose_random(self.char_vec.to_owned());
            self.score += 1;
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
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

        let text = vec![
            text::Line::from(vec![
                Span::from("Timer: "),
                Span::from(self.time.num_seconds().to_string()),
                Span::from("sec / 30 sec"),
            ]),
            text::Line::from(" "),
            text::Line::from(vec![
                Span::from("Value: "),
                self.random_char.to_string().yellow(),
            ]),
        ];
        let text2 = vec![
            text::Line::from(vec![Span::from("You score is: ")]),
            text::Line::from(" "),
            text::Line::from(vec![Span::from(self.score.to_string())]),
        ];
        if self.play {
            Paragraph::new(text)
                .centered()
                .block(block)
                .render(area, buf);
        } else {
            Paragraph::new(text2)
                .centered()
                .block(block)
                .render(area, buf);
        }
    }
}

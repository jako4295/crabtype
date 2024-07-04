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
    pub char_hist: [char; 3],
    pub char_future: [char; 3],

}

impl Default for GameLogic {
    fn default() -> GameLogic {
        let dict: Dict<bool> = get_dict();
        let start_t = Local::now();
        let load_char: Vec<char> = load_chars::load_files_to_vec(dict);

        GameLogic {
            time: Local::now().signed_duration_since(start_t),
            start_time: start_t,
            random_char: load_chars::chose_random(load_char.to_owned()),
            char_vec: load_char.to_owned(),
            score: 0,
            play: true,
            current_char: None,
            char_hist: [' ', ' ', ' '],
            char_future: [
                load_chars::chose_random(load_char.to_owned()), 
                load_chars::chose_random(load_char.to_owned()), 
                load_chars::chose_random(load_char.to_owned())
            ],
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
        self.char_hist = [' ', ' ', ' ']
    }
    pub fn compare_pressed_char(&mut self, character: char) {
        self.char_hist[0] = self.char_hist[1];
        self.char_hist[1] = self.char_hist[2];
        self.char_hist[2] = character;
        if self.time >= Duration::seconds(30) {
            self.play = false
        }
        if self.random_char == character {
            self.random_char = self.char_future[0];
            self.char_future[0] = self.char_future[1];
            self.char_future[1] = self.char_future[2];
            self.char_future[2] = load_chars::chose_random(self.char_vec.to_owned());
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
                ]),
            text::Line::from(vec![
                Span::styled(self.char_hist[0].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                Span::from(" "),
                Span::styled(self.char_hist[1].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                Span::from(" "),
                Span::styled(self.char_hist[2].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                Span::from("   "),
                self.random_char.to_string().yellow(),
                Span::from("   "),
                Span::styled(self.char_future[0].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                Span::from(" "),
                Span::styled(self.char_future[1].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                Span::from(" "),
                Span::styled(self.char_future[2].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
            ])
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
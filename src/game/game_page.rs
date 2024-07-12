use crate::char_lib::load_chars;
use crate::settings::settings_struct;
use chrono::{DateTime, Duration, Local};
use dict::{Dict, DictIface};

use ratatui::{
    prelude::*,
    symbols::border,
    text::Span,
    widgets::{block::*, *},
};

pub fn get_dict() -> Dict<bool> {
    let mut dict: Dict<bool> = Dict::<bool>::new();
    dict.add("letters".to_string(), true);
    dict.add("cap_letters".to_string(), false);
    dict.add("numbers".to_string(), false);
    dict
}

//pub fn load_settings() -> settings_struct::Settings {
//    // Load settings from .config/crabtype

//    settings_struct::Settings::default()
//}

fn _history() -> Vec<char> {
    todo!();
}

fn _future() -> Vec<char> {
    todo!();
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
    pub hist_amount: u8,
    pub future_amount: u8,
    pub char_hist: Vec<char>,
    pub char_future: Vec<char>,
    pub correct: bool,
    pub settings: settings_struct::Settings,
}

impl Default for GameLogic {
    fn default() -> GameLogic {
        let dict: Dict<bool> = get_dict();
        let start_t = Local::now();
        let load_char: Vec<char> = load_chars::load_files_to_vec(dict);
        let loaded_settings = settings_struct::Settings::read_config().unwrap();
        let mut h_vec = vec![];
        let mut f_vec = vec![];
        let h_amount: u8 = loaded_settings.history_length;
        let f_amount: u8 = loaded_settings.future_length;
        for _ in 0..h_amount {
            Vec::push(&mut h_vec, ' ');
        }
        for _ in 0..f_amount {
            f_vec.push(load_chars::chose_random(load_char.to_owned()));
        }

        GameLogic {
            time: Local::now().signed_duration_since(start_t),
            start_time: start_t,
            random_char: load_chars::chose_random(load_char.to_owned()),
            char_vec: load_char.to_owned(),
            score: 0,
            play: true,
            current_char: None,
            hist_amount: h_amount,
            future_amount: f_amount,
            char_hist: h_vec,
            char_future: f_vec,
            correct: true,
            settings: loaded_settings,
        }
    }
}

impl GameLogic {
    pub fn get_time(&mut self) {
        let time_now = Local::now();
        self.time = time_now.signed_duration_since(self.start_time);
    }
    pub fn reset_char_vec(&mut self) {
        self.char_future = vec![];
        self.char_hist = vec![];
        for _ in 0..self.hist_amount {
            self.char_hist.push(' ');
        }
        for _ in 0..self.future_amount {
            self.char_future
                .push(load_chars::chose_random(self.char_vec.to_owned()));
        }
    }
    pub fn reset(&mut self) {
        self.start_time = Local::now();
        self.score = 0;
        self.play = true;
        self.reset_char_vec();
    }

    pub fn compare_pressed_char(&mut self, character: char) {
        self.char_hist.remove(0);
        self.char_hist.push(character);

        if self.time >= Duration::seconds(self.settings.total_time_sec.into()) {
            self.play = false
        }
        if self.random_char == character {
            self.random_char = self.char_future[0];
            self.char_future.remove(0);
            self.char_future
                .push(load_chars::chose_random(self.char_vec.to_owned()));
            self.score += 1;
            self.correct = true;
        } else {
            self.correct = false;
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" CrabType ".bold());
        let instructions = Title::from(Line::from(vec![
            " Exit ".into(),
            " <Esc> ".yellow().bold(),
            " Restart ".into(),
            " <Space> ".yellow().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .border_set(border::THICK);

        let mut letter_line = vec![];

        for i in self.char_hist.clone() {
            letter_line.push(Span::styled(
                i.to_string(),
                Style::new().fg(Color::Rgb(128, 128, 128)),
            ));
            letter_line.push(Span::from(" "));
        }
        letter_line.push(Span::from("  "));
        letter_line.push(self.random_char.to_string().yellow());
        letter_line.push(Span::from("  "));
        for u in self.char_future.clone() {
            letter_line.push(Span::from(" "));
            letter_line.push(Span::styled(
                u.to_string(),
                Style::new().fg(Color::Rgb(128, 128, 128)),
            ));
        }

        let text = vec![
            text::Line::from(vec![
                Span::from("Timer: "),
                Span::from(self.time.num_seconds().to_string()),
                Span::from("sec / "),
                Span::from(self.settings.total_time_sec.to_string()),
                Span::from(" sec"),
            ]),
            text::Line::from(" "),
            text::Line::from(vec![Span::from("Value: ")]),
            text::Line::from(letter_line), // vec![
                                           // Span::styled(self.char_hist[0].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                                           // Span::from(" "),
                                           // Span::styled(self.char_hist[1].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                                           // Span::from(" "),
                                           // Span::styled(self.char_hist[2].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                                           // Span::from("   "),
                                           // self.random_char.to_string().yellow(),
                                           // Span::from("   "),
                                           // Span::styled(self.char_future[0].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                                           // Span::from(" "),
                                           // Span::styled(self.char_future[1].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),
                                           // Span::from(" "),
                                           // Span::styled(self.char_future[2].to_string(), Style::new().fg(Color::Rgb(128, 128, 128))),

                                           // ])
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
                .fg(Color::White)
                .render(area, buf);
        } else {
            Paragraph::new(text2)
                .centered()
                .block(block)
                .render(area, buf);
        }
    }
}

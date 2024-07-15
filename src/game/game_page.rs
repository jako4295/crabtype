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

        let hist_loop = if self.hist_amount == 0 {
            1
        } else {
            self.hist_amount
        };
        let future_loop = if self.future_amount == 0 {
            1
        } else {
            self.future_amount
        };

        for _ in 0..hist_loop {
            self.char_hist.push(' ');
        }
        for _ in 0..future_loop {
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
        if !self.play {
            return;
        }
        self.char_hist.remove(0);
        self.char_hist.push(character);

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

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" CrabType ".bold());
        let instructions = Title::from(Line::from(vec![" quit: <esc> | restart: <space> ".into()]));
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

        if self.time >= Duration::seconds(self.settings.total_time_sec.into()) {
            self.play = false
        }

        if self.play {
            self.render_game(area, buf, block);
        } else {
            self.render_result(area, buf, block);
        }
    }

    pub fn render_game(&self, area: Rect, buf: &mut Buffer, block: Block) {
        let mut letter_line = vec![];

        if self.hist_amount < self.future_amount {
            for _ in 0..(self.future_amount - self.hist_amount) {
                letter_line.push(Span::from("  "))
            }
        }

        if self.hist_amount != 0 {
            for i in self.char_hist.clone() {
                letter_line.push(Span::styled(
                    i.to_string(),
                    Style::new().fg(Color::Rgb(128, 128, 128)),
                ));
                letter_line.push(Span::from(" "));
            }
        }
        letter_line.push(Span::from("  "));
        letter_line.push(self.random_char.to_string().yellow());
        letter_line.push(Span::from("  "));

        if self.future_amount != 0 {
            for u in self.char_future.clone() {
                letter_line.push(Span::from(" "));
                letter_line.push(Span::styled(
                    u.to_string(),
                    Style::new().fg(Color::Rgb(128, 128, 128)),
                ));
            }
        }

        if self.hist_amount > self.future_amount {
            for _ in 0..(self.hist_amount - self.future_amount) {
                letter_line.push(Span::from("  "))
            }
        }

        let text = vec![
            text::Line::from(vec![Span::from(
                (i64::from(self.settings.total_time_sec) - self.time.num_seconds()).to_string(),
            )]),
            text::Line::from(" "),
            text::Line::from(vec![Span::from("Value: ")]),
            text::Line::from(letter_line),
        ];

        Paragraph::new(text)
            .centered()
            .block(block)
            .fg(Color::White)
            .render(area, buf);
    }

    pub fn render_result(&self, area: Rect, buf: &mut Buffer, block: Block) {
        let text2 = vec![
            text::Line::from(vec![Span::from("You score is: ")]),
            text::Line::from(" "),
            text::Line::from(vec![Span::from(self.score.to_string())]),
        ];
        Paragraph::new(text2)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

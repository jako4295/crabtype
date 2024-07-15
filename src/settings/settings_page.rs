use std::usize;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use super::settings_struct;

#[derive(Debug)]
pub enum SettingsStatus {
    Boolean(bool),
    Uint(u8),
}

#[derive(Debug)]
pub struct SettingsItem {
    pub description: String,
    pub status: SettingsStatus,
    pub reference_name: String,
}

impl SettingsItem {}

#[derive(Debug)]
pub struct SettingsStateList {
    pub state: TableState,
    pub items: Vec<SettingsItem>,
    pub last_selected: Option<usize>,
    pub settings_struct: settings_struct::Settings,
}

impl Default for SettingsStateList {
    fn default() -> Self {
        let settings = settings_struct::Settings::read_config().unwrap();

        let loaded_items = vec![
            SettingsItem {
                description: "Total game time (sec)".to_string(),
                status: SettingsStatus::Uint(settings.total_time_sec),
                reference_name: "total_time_sec".to_string(),
            },
            SettingsItem {
                description: "History Length".to_string(),
                status: SettingsStatus::Uint(settings.history_length),
                reference_name: "history_length".to_string(),
            },
            SettingsItem {
                description: "Future Length".to_string(),
                status: SettingsStatus::Uint(settings.future_length),
                reference_name: "future_length".to_string(),
            },
            SettingsItem {
                description: "Include lower case Letters".to_string(),
                status: SettingsStatus::Boolean(settings.lower_case_letters),
                reference_name: "lower_case_letters".to_string(),
            },
            SettingsItem {
                description: "Include capital Letters".to_string(),
                status: SettingsStatus::Boolean(settings.capital_letters),
                reference_name: "capital_letters".to_string(),
            },
            SettingsItem {
                description: "Include numbers".to_string(),
                status: SettingsStatus::Boolean(settings.numbers),
                reference_name: "numbers".to_string(),
            },
            SettingsItem {
                description: "Included parenthesis".to_string(),
                status: SettingsStatus::Boolean(settings.parenthesis),
                reference_name: "parenthesis".to_string(),
            },
            SettingsItem {
                description: "Indicate for 10 finger typing".to_string(),
                status: SettingsStatus::Boolean(settings.ten_finger_typing),
                reference_name: "ten_finger_typing".to_string(),
            },
            SettingsItem {
                description: "Hardcore mode".to_string(),
                status: SettingsStatus::Boolean(settings.hardcore),
                reference_name: "hardcore".to_string(),
            },
        ];

        let mut _state = TableState::default();
        if !loaded_items.is_empty() {
            _state.select(Some(0));
        }

        SettingsStateList {
            state: _state,
            items: loaded_items,
            last_selected: None,
            settings_struct: settings,
        }
    }
}

impl SettingsItem {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::from(self.description.clone()),
            Cell::from(self.settings_status_to_str()),
        ])
    }

    fn settings_status_to_str(&self) -> String {
        match self.status {
            SettingsStatus::Uint(val) => val.to_string(),
            SettingsStatus::Boolean(val) => {
                if val {
                    "On".to_string()
                } else {
                    "Off".to_string()
                }
            }
        }
    }
}

impl SettingsStateList {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![
            " quit: <esc> | down: ↓ or j | up: ↑ or k | toggle increase: → or l | toggle decrease: ← or h ".into(),
        ]));

        let block = Block::default()
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .fg(Color::Blue)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .border_set(border::THICK)
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            );

        let rows: Vec<Row> = self.items.iter().map(|item| item.to_table_row()).collect();

        let table = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(50),
                Constraint::Min(10),
                Constraint::Min(10),
            ],
        )
        .header(
            Row::new(vec!["Description", "Status"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(block)
        .widths([Constraint::Percentage(90), Constraint::Percentage(10)])
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol(">");

        StatefulWidget::render(table, area, buf, &mut self.state);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn edit_entry(&mut self, increment: bool) {
        let i = self.state.selected();
        let selected_item = &mut self.items[i.unwrap()];

        match &mut selected_item.status {
            SettingsStatus::Boolean(ref mut val) => {
                *val = !*val;
            }
            SettingsStatus::Uint(ref mut val) => {
                if increment {
                    if *val < 255 {
                        *val += 1;
                    } else {
                        *val = 0;
                    }
                } else if *val > 0 {
                    *val -= 1;
                } else {
                    *val = 255;
                }
            }
        }

        let new_settings = self.get_settings_struct();
        let _ = settings_struct::Settings::write_config(&new_settings);
    }

    fn get_settings_struct(&mut self) -> settings_struct::Settings {
        let mut settings = settings_struct::Settings::default();

        for item in &self.items {
            match item.reference_name.as_str() {
                "total_time_sec" => {
                    if let SettingsStatus::Uint(val) = item.status {
                        settings.total_time_sec = val;
                    }
                }
                "history_length" => {
                    if let SettingsStatus::Uint(val) = item.status {
                        settings.history_length = val;
                    }
                }
                "future_length" => {
                    if let SettingsStatus::Uint(val) = item.status {
                        settings.future_length = val;
                    }
                }
                "lower_case_letters" => {
                    if let SettingsStatus::Boolean(val) = item.status {
                        settings.lower_case_letters = val;
                    }
                }
                "capital_letters" => {
                    if let SettingsStatus::Boolean(val) = item.status {
                        settings.capital_letters = val;
                    }
                }
                "numbers" => {
                    if let SettingsStatus::Boolean(val) = item.status {
                        settings.numbers = val;
                    }
                }
                "parenthesis" => {
                    if let SettingsStatus::Boolean(val) = item.status {
                        settings.parenthesis = val;
                    }
                }
                "ten_finger_typing" => {
                    if let SettingsStatus::Boolean(val) = item.status {
                        settings.ten_finger_typing = val;
                    }
                }
                "hardcore" => {
                    if let SettingsStatus::Boolean(val) = item.status {
                        settings.hardcore = val;
                    }
                }
                _ => {}
            }
        }
        settings
    }
}

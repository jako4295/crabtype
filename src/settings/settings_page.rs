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
}

#[derive(Debug)]
pub struct SettingsStateList {
    pub state: ListState,
    pub items: Vec<SettingsItem>,
    pub last_selected: Option<usize>,
}

impl Default for SettingsStateList {
    fn default() -> Self {
        let settings = settings_struct::Settings::read_config().unwrap();

        let loaded_items = vec![
            SettingsItem {
                description: "Total game time (sec)".to_string(),
                status: SettingsStatus::Uint(settings.total_time_sec),
            },
            SettingsItem {
                description: "History Length".to_string(),
                status: SettingsStatus::Uint(settings.history_length),
            },
            SettingsItem {
                description: "Future Length".to_string(),
                status: SettingsStatus::Uint(settings.future_length),
            },
            SettingsItem {
                description: "Include capital Letters".to_string(),
                status: SettingsStatus::Boolean(settings.capital_letters),
            },
            SettingsItem {
                description: "Include numbers".to_string(),
                status: SettingsStatus::Boolean(settings.numbers),
            },
            SettingsItem {
                description: "Included parenthesis".to_string(),
                status: SettingsStatus::Boolean(settings.parenthesis),
            },
            SettingsItem {
                description: "Indicate for 10 finger typing".to_string(),
                status: SettingsStatus::Boolean(settings.ten_finger_typing),
            },
            SettingsItem {
                description: "Hardcore mode".to_string(),
                status: SettingsStatus::Boolean(settings.hardcore),
            },
        ];

        let mut _state = ListState::default();
        if !loaded_items.is_empty() {
            _state.select(Some(0));
        }

        SettingsStateList {
            state: _state,
            items: loaded_items,
            last_selected: None,
        }
    }
}

impl SettingsItem {
    fn to_list_item(&self) -> ListItem {
        ListItem::new(Line::from(vec![
            Span::raw(self.description.to_string()),
            Span::raw(self.settings_status_to_str().to_string()),
        ]))
    }

    fn settings_status_to_str(&self) -> String {
        match self.status {
            SettingsStatus::Uint(val) => val.to_string(),
            SettingsStatus::Boolean(val) => boolean_translator(val),
        }
    }
}

impl SettingsStateList {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK);

        // let crabtype: String = "settings".to_string();

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(_i, todo_item)| todo_item.to_list_item())
            .collect();

        let items = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(Color::White)
                    .bg(Color::Blue),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // self.state.select(Some(0));

        StatefulWidget::render(items, area, buf, &mut self.state);
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

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }

    // TODO:
    //  1. Write save to config method
    //  2. Write load method
    //  2. Write toggle change method:
    //    a. Handle booleans as change by pressing enter
    //    b. Handles u8 as a pop up where it is possible
    //       to change the value. If invalid u8 is typed
    //       then go back to previous state.
}

fn boolean_translator(state: bool) -> String {
    if state {
        "On".to_string()
    } else {
        "Off".to_string()
    }
}

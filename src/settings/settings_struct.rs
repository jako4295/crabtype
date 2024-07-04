#[derive(Debug)]
pub struct Settings {
    pub total_time_sec: u8,
    pub history_length: u8,
    pub future_length: u8,
    pub capital_letters: bool,
    pub numbers: bool,
    pub parenthesis: bool,
    pub ten_finger_typing: bool,

    // TODO: remove time aspect and create stay alive mode.
    // Should exit game if character/minute is too slow or
    // if a wrong word is typed:
    pub hardcore: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            total_time_sec: 30,
            history_length: 3,
            future_length: 3,
            capital_letters: false,
            numbers: false,
            parenthesis: false,
            ten_finger_typing: false,

            // TODO: remove time aspect and create stay alive mode.
            // Should exit game if character/minute is too slow or
            // if a wrong word is typed:
            hardcore: false,
        }
    }
}

use confy;
static APP_NAME: &str = "crabtype";
static CONFIG_NAME: &str = "config";

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Copy)]
pub struct Settings {
    pub total_time_sec: u8,
    pub history_length: u8,
    pub future_length: u8,
    pub lower_case_letters: bool,
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
            lower_case_letters: true,
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

impl Settings {
    pub fn write_config(cfg: &Settings) -> Result<(), confy::ConfyError> {
        confy::store(APP_NAME, CONFIG_NAME, cfg)
    }
    pub fn read_config() -> Result<Settings, confy::ConfyError> {
        let cfg = confy::load(APP_NAME, CONFIG_NAME);

        // TODO: Make it more obvious if a bad config has been read
        match cfg {
            Ok(config) => Ok(config),
            Err(_) => Ok(Settings::default()),
        }
    }
}

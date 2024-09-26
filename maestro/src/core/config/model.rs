use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    setting1: String,
    setting2: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            setting1: "default_value".to_string(),
            setting2: 42,
        }
    }
}

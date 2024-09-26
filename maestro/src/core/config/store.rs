use std::fs::File;
use std::path::Path;
use std::io::{BufReader};
use crate::core::config::model::Config;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, String> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|err| format!("Failed to open file: {}", err))?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader).map_err(|err| format!("Failed to parse JSON: {}", err))?;
    Ok(config)
}
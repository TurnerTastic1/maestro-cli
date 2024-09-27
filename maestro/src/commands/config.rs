use std::fs;
use std::path::PathBuf;
use inquire::Text;
use maestro_core::core::config::store::save_config;

pub fn handle_config() -> Result<bool, String> {
    // Prompt for the configuration file path
    let config_file_path = Text::new("Path to the configuration file")
        .with_default("~/maestro-config.json")
        .prompt()
        .map_err(|err| err);

    match config_file_path {
        Ok(path) => {
            // Convert the path to an absolute path
            let absolute_path = fs::canonicalize(PathBuf::from(path.to_string()))
                .map_err(|err| format!("Failed to canonicalize path '{}': {}\nEnsure the file exists", path, err))?;

            match save_config(absolute_path.to_str().unwrap().to_string()) {
                Ok(path) => {
                    println!("Configuration file set to: {}", path);
                }
                Err(err) => {
                    println!("Failed to set configuration file: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Failed to read configuration file path: {}", err);
        }
    }

    Ok(false)
}
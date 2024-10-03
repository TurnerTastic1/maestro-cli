use inquire::Text;
use maestro_core::core::config::store::save_user_config_file;

pub fn handle_config() -> Result<bool, String> {
    // Prompt for the configuration file path
    let config_file_path = Text::new("Path to the configuration file")
        .with_default("~/maestro-config.json")
        .prompt()
        .map_err(|err| err);

    match config_file_path {
        Ok(path) => {
            match save_user_config_file(path) {
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
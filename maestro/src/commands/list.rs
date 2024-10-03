use maestro_core::core::config::store::load_config;

pub fn handle_list() -> Result<bool, String> {
    let load_result = load_config();

    match load_result {
        Ok(maestro) => {
            println!("Projects:");
            for workspace in &maestro.workspaces {
                println!("  - {}", workspace.name);
            }
            Ok(false)
        }
        Err(err) => {
            println!("Failed to load configuration: {}", err);
            Ok(false)
        }
    }
}
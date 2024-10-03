use colored::Colorize;
use maestro_core::core::config::store::load_config;

pub fn handle_list() -> Result<bool, String> {
    let load_result = load_config();

    match load_result {
        Ok(maestro) => {
            println!("{:<20} {:<40} {:<10}", "Name".bold(), "Description".bold(), "Status".bold());
            println!("{} {} {}", "-".repeat(20), "-".repeat(40), "-".repeat(10));
            for workspace in &maestro.workspaces {
                println!("{:<20} {:<40} {:<10}", workspace.name, workspace.description, "Active");
            }
            Ok(false)
        }
        Err(err) => {
            println!("Failed to load configuration: {}", err);
            Ok(false)
        }
    }
}
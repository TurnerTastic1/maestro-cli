use maestro_core::core::config::store::load_config;
pub fn handle_config() -> Result<bool, String> {
    println!("Configuring ...");
    load_config("config.json")?;
    Ok(false)
}
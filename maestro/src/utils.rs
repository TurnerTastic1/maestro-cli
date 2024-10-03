use std::io::{self, Write};
use colored::Colorize;

pub fn readline() -> Result<String, String> {
    print!("{}", "maestro$ ".bold().blue());
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

use std::io::{self, Write};

pub fn readline() -> Result<String, String> {
    print!("maestro$ ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

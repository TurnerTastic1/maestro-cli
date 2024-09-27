mod commands;
mod utils;

fn main() -> Result<(), String> {
    loop {
        let line = utils::readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match commands::respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                eprintln!("{err}");
            }
        }
    }

    Ok(())
}

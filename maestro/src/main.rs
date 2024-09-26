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

// fn respond(line: &str) -> Result<bool, String> {
//     let args = shlex::split(line).ok_or("error: Invalid quoting")?;
//     let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
//     match cli.command {
//         Commands::Ping => {
//             write!(std::io::stdout(), "Pong").map_err(|e| e.to_string())?;
//             std::io::stdout().flush().map_err(|e| e.to_string())?;
//         }
//         Commands::Exit => {
//             write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
//             std::io::stdout().flush().map_err(|e| e.to_string())?;
//             return Ok(true);
//         }
//     }
//     Ok(false)
// }
//
// #[derive(Debug, Parser)]
// #[command(multicall = true)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }
//
// #[derive(Debug, Subcommand)]
// enum Commands {
//     Ping,
//     Exit,
// }
//
// fn readline() -> Result<String, String> {
//     write!(std::io::stdout(), "maestro $ ").map_err(|e| e.to_string())?;
//     std::io::stdout().flush().map_err(|e| e.to_string())?;
//     let mut buffer = String::new();
//     std::io::stdin()
//         .read_line(&mut buffer)
//         .map_err(|e| e.to_string())?;
//     Ok(buffer)
// }
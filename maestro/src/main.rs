// use std::io::Write;

// use clap::{Parser, Subcommand};

// /// A fictional versioning CLI
// #[derive(Debug, Parser)] // requires `derive` feature
// #[command(name = "git")]
// #[command(about = "A microservice monitoring and management tool", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Debug, Subcommand)]
// enum Commands {
//     /// echos strings
//     #[command(arg_required_else_help = true)]
//     Echo {
//         /// The input to echo
//         input: String,
//     },
//     Ping,
//     Exit,
// }

// fn respond(line: &str) -> Result<bool, String> {
//   let args = shlex::split(line).ok_or("error: Invalid quoting")?;
//   let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
//   match cli.command {
//       Commands::Echo { input } => {
//           write!(std::io::stdout(), "{}", input).map_err(|e| e.to_string())?;
//           std::io::stdout().flush().map_err(|e| e.to_string())?;
//       }
//       Commands::Ping => {
//           write!(std::io::stdout(), "Pong").map_err(|e| e.to_string())?;
//           std::io::stdout().flush().map_err(|e| e.to_string())?;
//       }
//       Commands::Exit => {
//           write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
//           std::io::stdout().flush().map_err(|e| e.to_string())?;
//           return Ok(true);
//       }
//   }
//   Ok(false)
// }

// fn readline() -> Result<String, String> {
//   write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
//   std::io::stdout().flush().map_err(|e| e.to_string())?;
//   let mut buffer = String::new();
//   std::io::stdin()
//       .read_line(&mut buffer)
//       .map_err(|e| e.to_string())?;
//   Ok(buffer)
// }

// fn main() -> Result<(), String> {
//   loop {
//       let line = readline()?;
//       let line = line.trim();
//       if line.is_empty() {
//           continue;
//       }

//       match respond(line) {
//           Ok(quit) => {
//               if !quit {
//                   break;
//               }
//           }
//           Err(err) => {
//               write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
//               std::io::stdout().flush().map_err(|e| e.to_string())?;
//           }
//       }
//   }

//   Ok(())
// }


use std::io::Write;

use clap::{Parser, Subcommand};

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Ping => {
            write!(std::io::stdout(), "Pong").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Exit => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
    }
    Ok(false)
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ping,
    Exit,
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
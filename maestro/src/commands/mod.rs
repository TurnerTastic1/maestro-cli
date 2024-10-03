mod ping;
mod exit;
mod config;
mod list;

use clap::Parser;
use crate::commands::config::handle_config;
use crate::commands::ping::handle_ping;
use crate::commands::exit::handle_exit;
use crate::commands::list::handle_list;

#[derive(Debug, Parser)]
#[command(multicall = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Ping,
    Exit,
    Config,
    List,
}

pub fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;

    match cli.command {
        Commands::Ping => handle_ping(),
        Commands::Exit => handle_exit(),
        Commands::Config => handle_config(),
        Commands::List => handle_list(),
    }
}
use bot::start;
use clap::{Arg, ArgAction, Command};
use error_stack::{Context, Report, Result, ResultExt};
use std::{env, fmt};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod bot;
mod commands;
mod config;
mod event_handler;
mod hash_string;

#[derive(Debug)]
struct ApplicationInitialisationError;

impl fmt::Display for ApplicationInitialisationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to start application")
    }
}

impl Context for ApplicationInitialisationError {}

#[tokio::main]
async fn main() -> Result<(), ApplicationInitialisationError> {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .init();

    let commands = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("start").about("Starts the bot"))
        .subcommand(Command::new("push").about("Pushes the latest commands to Discord."))
        .subcommand(
            Command::new("setup")
                .about("Initialise configuration files.")
                .arg(
                    Arg::new("token")
                        .required(true)
                        .action(ArgAction::Set)
                        .help("Discord bot token"),
                ),
        );

    match commands.get_matches().subcommand() {
        Some(("start", _)) => start()
            .await
            .change_context(ApplicationInitialisationError)?,
        Some(("push", _)) => start()
            .await
            .change_context(ApplicationInitialisationError)?,
        Some(("setup", args)) => {
            let token: &String = match args.get_one("token") {
                Some(token) => token,
                None => {
                    return Err(Report::new(ApplicationInitialisationError)
                        .attach_printable("Failed to parse token from command line input"))
                }
            };

            let config = config::ConfigFile::new(token, vec![1110009506391404616]);

            config
                .write()
                .change_context(ApplicationInitialisationError)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

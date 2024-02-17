use crate::{config::ConfigFile, event_handler::BotEvents};
use error_stack::{Context, Report, Result, ResultExt};
use serenity::{all::GatewayIntents, Client};
use std::fmt;
use tracing::{info, trace};

#[derive(Debug)]
pub struct BotStartError;

impl fmt::Display for BotStartError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to start Discord bot")
    }
}

impl Context for BotStartError {}

pub async fn start() -> Result<(), BotStartError> {
    trace!("Reading config file :3");
    let config = ConfigFile::read().map_err(|e| {
        Report::from(e)
            .attach_printable("Failed to read config file.")
            .change_context(BotStartError)
    })?;

    let token = config.discord_token;

    trace!("Read DISCORD_TOKEN from config file: {}", token);

    let gateway_intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    info!("Starting Discord bot");
    let mut client = Client::builder(token, gateway_intents)
        .event_handler(BotEvents::default())
        .await
        .change_context(BotStartError)?;

    client.start().await.change_context(BotStartError)?;

    Ok(())
}

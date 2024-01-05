use crate::event_handler::BotEvents;
use dotenvy::dotenv;
use error_stack::{Context, Result, ResultExt};
use serenity::{all::GatewayIntents, Client};
use std::{env, fmt};
use tracing::{info, trace, warn, error};

#[derive(Debug)]
pub struct BotStartError;

impl fmt::Display for BotStartError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to start Discord bot")
    }
}

impl Context for BotStartError {}

pub async fn start() -> Result<(), BotStartError> {
    if let Err(e) = dotenv() {
        info!("No .env file found or failed to read .env file");
        error!("dotenvy error: {}", e);
        warn!("Ensure environment variables are set in shell")
    }

    trace!("Reading DISCORD_TOKEN from environment variables");

    let token = env::var("DISCORD_TOKEN")
        .change_context(BotStartError)
        .attach_printable("Failed to read DISCORD_TOKEN from environment variables")?;

    trace!("Read DISCORD_TOKEN from environment variables: {}", token);

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

use crate::event_handler::BotEvents;
use async_trait::async_trait;
use error_stack::Context;
use serenity::{
    builder::CreateCommand, client::Context as SerenityContext,
    model::application::CommandInteraction,
};
use std::fmt;

mod ban;

#[async_trait]
pub trait Command
where
    Self: CommandInfo,
{
    async fn execute<'a>(
        &self,
        handler: &BotEvents,
        ctx: &SerenityContext,
        interaction: &'a mut CommandInteraction,
    ) -> error_stack::Result<(), CommandExecutionError>;

    fn register(&self) -> CreateCommand;
}

pub trait CommandInfo {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[derive(Debug)]
pub struct CommandExecutionError;

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error whilst executing command")
    }
}

impl Context for CommandExecutionError {}

pub fn load_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![Box::new(ban::BanCommand)]
}

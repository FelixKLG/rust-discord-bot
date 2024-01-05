use crate::commands::{self, Command};
use async_trait::async_trait;
use serenity::{
    client::{Context, EventHandler},
    model::{application::Interaction, gateway, id::GuildId},
};
use tracing::{error, info};

pub struct BotEvents {
    pub commands: Vec<Box<dyn for<'a> Command + Send + Sync>>,
}

#[async_trait]
impl EventHandler for BotEvents {
    async fn ready(&self, ctx: Context, ready: gateway::Ready) {
        info!("Connected to Discord");

        let discriminator = match ready.user.discriminator {
            Some(discriminator) => discriminator.get(),
            None => 0u16,
        };

        info!("Username: {}#{}", ready.user.name, discriminator);
        info!("User Id: {}", ready.user.id);

        // change this to ur guild id, will make better way later.
        let guild = GuildId::new(1110009506391404616);

        let command_vec = self
            .commands
            .iter()
            .map(|command| command.register())
            .collect::<Vec<_>>();

        guild.set_commands(&ctx.http, command_vec).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(mut interaction_command) => {
                for command in &self.commands {
                    if &command.name() == &interaction_command.data.name.as_str() {
                        match command.execute(self, &ctx, &mut interaction_command).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("Failed to execute command: {:?}", err);
                            }
                        };
                    }
                }
            }
            _ => {}
        }
    }
}

impl Default for BotEvents {
    fn default() -> Self {
        Self {
            commands: commands::load_commands(),
        }
    }
}

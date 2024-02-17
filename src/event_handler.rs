use crate::{
    bot::BotStartError,
    commands::{self, Command},
    config::ConfigFile,
};
use async_trait::async_trait;
use error_stack::ResultExt;
use serenity::{
    client::{Context, EventHandler},
    model::{application::Interaction, gateway, id::GuildId},
};
use tracing::{error, info};

pub struct BotEvents {
    pub commands: Vec<Box<dyn for<'a> Command + Send + Sync>>,
    pub http: reqwest::Client,
    pub cfg: ConfigFile,
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

        for g in &self.cfg.guilds {
            let guild = GuildId::new(g.clone() as u64);

            let command_vec = self
                .commands
                .iter()
                .map(|command| command.register())
                .collect::<Vec<_>>();

            guild.set_commands(&ctx.http, command_vec).await.unwrap();
        }
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
            http: reqwest::Client::new(),
            cfg: ConfigFile::read().change_context(BotStartError).unwrap(),
        }
    }
}

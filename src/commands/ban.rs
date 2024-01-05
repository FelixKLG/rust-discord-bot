use super::{Command, CommandExecutionError, CommandInfo};
use crate::event_handler::BotEvents;
use async_trait::async_trait;
use error_stack::{Report, Result};
use serenity::{
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::{
        application::{CommandDataOptionValue, CommandInteraction, CommandOptionType},
        Permissions,
    },
};

#[derive(Debug)]
pub struct BanCommand;

impl CommandInfo for BanCommand {
    fn name(&self) -> String {
        String::from("ban")
    }

    fn description(&self) -> String {
        String::from("Ban a user from the server")
    }
}

#[async_trait]
impl Command for BanCommand {
    async fn execute<'a>(
        &self,
        _handler: &BotEvents,
        ctx: &Context,
        interaction: &'a mut CommandInteraction,
    ) -> Result<(), CommandExecutionError> {
        let interaction_member = match &interaction.member {
            Some(member) => member,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to fetch member from interaction"))
            }
        };

        // Target from command args
        let target_command_data = match interaction.data.options.get(0) {
            Some(target_command_data) => target_command_data,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get command arg data user"))
            }
        };

        // Target from command args as user
        let target_user = match target_command_data.value {
            CommandDataOptionValue::User(user_id) => user_id,
            _ => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get target user arg"))
            }
        };

        // Target to User from UserId
        let target_user = target_user.to_user(&ctx.http).await.map_err(|e| {
            Report::from(e)
                .change_context(CommandExecutionError)
                .attach_printable("Failed to fetch user")
        })?;

        // Get guild from command data
        let guild = match interaction.guild_id {
            Some(guild_id) => guild_id
                .to_partial_guild(&ctx.http)
                .await
                .map_err(|e| Report::from(e).change_context(CommandExecutionError))?,
            None => return Err(Report::from(CommandExecutionError))?,
        };

        if target_user.id == interaction.user.id {
            let message = CreateInteractionResponseMessage::new()
                .content("You're unable to ban yourself...")
                .ephemeral(true);

            let builder = CreateInteractionResponse::Message(message);

            interaction
                .create_response(&ctx.http, builder)
                .await
                .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

            return Ok(());
        }

        if guild.owner_id == target_user.id {
            let message = CreateInteractionResponseMessage::new()
                .content("Unable to ban server owner")
                .ephemeral(true);

            let builder = CreateInteractionResponse::Message(message);

            interaction
                .create_response(&ctx.http, builder)
                .await
                .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

            return Ok(());
        }

        // Target from command args as member
        let target_member = guild
            .member(&ctx.http, &target_user)
            .await
            .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

        if let Some((_, target_role_pos)) = target_member.highest_role_info(&ctx.cache) {
            if let Some((_, member_role_pos)) = interaction_member.highest_role_info(&ctx.cache) {
                if target_role_pos >= member_role_pos
                    && (guild.owner_id != interaction_member.user.id)
                {
                    let message = CreateInteractionResponseMessage::new()
                        .content("You lack sufficient privileges to ban this user")
                        .ephemeral(true);

                    let builder = CreateInteractionResponse::Message(message);

                    interaction
                        .create_response(&ctx.http, builder)
                        .await
                        .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

                    return Ok(());
                }
            }
        };

        // ban user
        match interaction.data.options.get(1) {
            Some(target_command_data) => {
                let ban_reason = match &target_command_data.value {
                    CommandDataOptionValue::String(ban_reason) => ban_reason,
                    _ => {
                        return Err(Report::from(CommandExecutionError)
                            .attach_printable("Failed to get ban reason arg"))
                    }
                };

                guild
                    .ban_with_reason(&ctx.http, &target_user, 0, ban_reason)
                    .await
                    .map_err(|e| Report::from(e).change_context(CommandExecutionError))?
            }
            None => {
                guild
                    .ban(&ctx.http, &target_user, 0)
                    .await
                    .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;
            }
        };

        // respond to command
        let message = CreateInteractionResponseMessage::new()
            .content(format!(
                "Banned {}",
                target_user.global_name.unwrap_or(target_user.name)
            ))
            .ephemeral(true);
        let interaction_response = CreateInteractionResponse::Message(message);

        interaction
            .create_response(&ctx.http, interaction_response)
            .await
            .map_err(|e| {
                Report::from(e)
                    .change_context(CommandExecutionError)
                    .attach_printable("Failed to reply to message")
            })?;

        Ok(())
    }

    fn register(&self) -> CreateCommand {
        return CreateCommand::new(self.name())
            .description(self.description())
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "user", "The user to ban")
                    .required(true),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "reason",
                "Reason to ban the user",
            ))
            .default_member_permissions(Permissions::BAN_MEMBERS)
            .dm_permission(false);
    }
}

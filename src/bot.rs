use anyhow::{Context as ContextErr, Result};
use serenity::{
    async_trait,
    http::Http,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction,
            InteractionResponseType::{ChannelMessageWithSource, UpdateMessage},
        },
        prelude::{ChannelId, Message},
    },
};
use crate::{message::{MessageBuilder, Attachment}, MessageUtil};
type Command = ApplicationCommandInteraction;

#[async_trait]
pub trait BotUtil {
    async fn answer(&self, command: &Command, message: MessageBuilder) -> Result<Message>;

    async fn edit_response(&self, command: &Command, message: MessageBuilder) -> Result<()>;

    async fn followup(&self, command: &Command, message: MessageBuilder) -> Result<()>;

    async fn send(&self, channel_id: ChannelId, message: MessageBuilder) -> Result<Message>;
}

#[async_trait]
impl BotUtil for Http {
    async fn answer(&self, command: &Command, message: MessageBuilder) -> Result<Message> {
        (command
            .create_interaction_response(self, |response| {
                response
                    .kind(ChannelMessageWithSource)
                    .interaction_response_data(|answer| {
                        answer.ephemeral(message.ephemeral);
                        answer.allowed_mentions(|mentions| mentions.empty_users());
                        answer.content(message.content);
                        message.files.iter().for_each(|Attachment { file, filename }| {
                            answer.add_file((file.as_slice(), filename.as_str()));
                        });
                        answer.set_buttons(message.buttons);
                        answer
                    })
            }).await)
        .context("Command create response failed")?;
        Ok(command.get_interaction_response(self).await?)
    }

    async fn edit_response(&self, command: &Command, message: MessageBuilder) -> Result<()> {
        (command
            .create_interaction_response(self, |response| {
                response.kind(UpdateMessage)
                .interaction_response_data(|answer| {
                    answer.allowed_mentions(|mentions| mentions.empty_users());
                    answer.content(message.content);
                    message.files.iter().for_each(|Attachment { file, filename }| {
                        answer.add_file((file.as_slice(), filename.as_str()));
                    });
                    answer.set_buttons(message.buttons);
                    answer
                })
            }).await)
        .context("Command edit message response failed")?;
        Ok(())
    }

    async fn followup(&self, command: &Command, message: MessageBuilder) -> Result<()> {
        (command
            .create_followup_message(self, |answer| {
                answer.ephemeral(message.ephemeral);
                answer.allowed_mentions(|mentions| mentions.empty_users());
                answer.content(message.content);
                message.files.iter().for_each(|Attachment { file, filename }| {
                    answer.add_file((file.as_slice(), filename.as_str()));
                });
                answer.set_buttons(message.buttons);
                answer
            })
            .await)
            .context("Command create followup failed")?;
        Ok(())
    }

    async fn send(&self, channel_id: ChannelId, message: MessageBuilder) -> Result<Message> {
        Ok(
            (channel_id.send_message(
                &self, |answer| {
                    answer.allowed_mentions(|mentions| mentions.empty_users());
                    answer.content(message.content);
                    message.files.iter().for_each(|Attachment { file, filename }| {
                        answer.add_file((file.as_slice(), filename.as_str()));
                    });
                    answer.set_buttons(message.buttons);
                    answer
                }
            ).await).context("Failed to send message")?
        )
    }
}

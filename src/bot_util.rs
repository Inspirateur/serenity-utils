use anyhow::{Context as ContextErr, Result};
use serenity::{
    async_trait,
    http::Http,
    model::prelude::{ChannelId, Message},
};
use crate::{message::{MessageBuilder, Attachment}, MessageUtil};

#[async_trait]
pub trait BotUtil {
    async fn send(&self, channel_id: ChannelId, message: MessageBuilder) -> Result<Message>;
}

#[async_trait]
impl BotUtil for Http {
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
use anyhow::{Context as ContextErr, Result};
use serenity::{
    all::CreateAllowedMentions, builder::CreateMessage, http::Http, model::prelude::{ChannelId, Message}
};
use crate::message_util::MessageBuilder;

pub trait BotUtil {
    async fn send(&self, channel_id: ChannelId, message: MessageBuilder) -> Result<Message>;
}

impl BotUtil for Http {
    async fn send(&self, channel_id: ChannelId, message: MessageBuilder) -> Result<Message> {
        Ok(
            (channel_id.send_message(
                &self,
                CreateMessage::default()
                    .allowed_mentions(CreateAllowedMentions::default().empty_users())
                    .content(message.content.clone())
                    .add_files(message.files)
                    .components(message.buttons)
            ).await).context("Failed to send message")?
        )
    }
}
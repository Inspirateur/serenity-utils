use anyhow::{Context as ContextErr, Result};
use serenity::{
    all::CreateAllowedMentions, builder::{CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage}, http::Http, model::{
        application::{
            CommandInteraction, ComponentInteraction, InteractionResponseFlags, ModalInteraction
        },
        prelude::Message
    }
};
use crate::message_util::MessageBuilder;


pub trait CommandUtil {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseFlags) -> Result<Message>;

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()>;
}


impl CommandUtil for CommandInteraction {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseFlags) -> Result<Message> {
        (self
            .create_response(http, 
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(message.ephemeral)
                        .allowed_mentions(CreateAllowedMentions::new().empty_users())
                        .content(message.content)
                        .add_files(message.files)
                        .components(message.buttons)
                        .flags(kind)
                )).await)
        .context("Command create response failed")?;
        Ok(self.get_response(http).await?)
    }

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()> {
        (self
            .create_followup(http, 
                CreateInteractionResponseFollowup::new()
                    .ephemeral(message.ephemeral)
                    .allowed_mentions(CreateAllowedMentions::new().empty_users())
                    .content(message.content)
                    .add_files(message.files)
                    .components(message.buttons)
            ).await)
            .context("Command create followup failed")?;
        Ok(())
    }
}


impl CommandUtil for ComponentInteraction {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseFlags) -> Result<Message> {
        (self
            .create_response(http, 
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(message.ephemeral)
                        .allowed_mentions(CreateAllowedMentions::new().empty_users())
                        .content(message.content)
                        .add_files(message.files)
                        .components(message.buttons)
                        .flags(kind)
                )).await)
        .context("Command create response failed")?;
        Ok(self.get_response(http).await?)
    }

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()> {
        (self
            .create_followup(http, 
                CreateInteractionResponseFollowup::new()
                    .ephemeral(message.ephemeral)
                    .allowed_mentions(CreateAllowedMentions::new().empty_users())
                    .content(message.content)
                    .add_files(message.files)
                    .components(message.buttons)
            ).await)
            .context("Command create followup failed")?;
        Ok(())
    }
}


impl CommandUtil for ModalInteraction {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseFlags) -> Result<Message> {
        (self
            .create_response(http, 
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(message.ephemeral)
                        .allowed_mentions(CreateAllowedMentions::new().empty_users())
                        .content(message.content)
                        .add_files(message.files)
                        .components(message.buttons)
                        .flags(kind)
                )).await)
        .context("Command create response failed")?;
        Ok(self.get_response(http).await?)
    }

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()> {
        (self
            .create_followup(http, 
                CreateInteractionResponseFollowup::new()
                    .ephemeral(message.ephemeral)
                    .allowed_mentions(CreateAllowedMentions::new().empty_users())
                    .content(message.content)
                    .add_files(message.files)
                    .components(message.buttons)
            ).await)
            .context("Command create followup failed")?;
        Ok(())
    }
}
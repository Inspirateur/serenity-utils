use anyhow::{Context as ContextErr, Result};
use serenity::{
    async_trait,
    http::Http,
    model::{
        application::interaction::{
            InteractionResponseType,
            modal::ModalSubmitInteraction,
            application_command::ApplicationCommandInteraction,
            message_component::MessageComponentInteraction
        },
        prelude::Message
    },
};
use crate::{message::{MessageBuilder, Attachment}, MessageUtil};


#[async_trait]
pub trait CommandUtil {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseType) -> Result<Message>;

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()>;
}


#[async_trait]
impl CommandUtil for ApplicationCommandInteraction {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseType) -> Result<Message> {
        (self
            .create_interaction_response(http, |response| {
                response
                    .kind(kind)
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
        Ok(self.get_interaction_response(http).await?)
    }

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()> {
        (self
            .create_followup_message(http, |answer| {
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
}


#[async_trait]
impl CommandUtil for MessageComponentInteraction {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseType) -> Result<Message> {
        (self
            .create_interaction_response(http, |response| {
                response
                    .kind(kind)
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
        Ok(self.get_interaction_response(http).await?)
    }

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()> {
        (self
            .create_followup_message(http, |answer| {
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
}


#[async_trait]
impl CommandUtil for ModalSubmitInteraction {
    async fn response(&self, http: &Http, message: MessageBuilder, kind: InteractionResponseType) -> Result<Message> {
        (self
            .create_interaction_response(http, |response| {
                response
                    .kind(kind)
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
        Ok(self.get_interaction_response(http).await?)
    }

    async fn followup(&self, http: &Http, message: MessageBuilder) -> Result<()> {
        (self
            .create_followup_message(http, |answer| {
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
}
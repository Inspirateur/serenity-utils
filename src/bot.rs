use anyhow::{Context as ContextErr, Result};
use serenity::{
    async_trait,
    http::Http,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction,
            InteractionResponseType::ChannelMessageWithSource,
        },
        prelude::{ChannelId, Message},
    },
};
use crate::message::{MessageBuilder, Attachment};
type Command = ApplicationCommandInteraction;

#[async_trait]
pub trait Bot {
    async fn answer(&self, command: &Command, message: MessageBuilder) -> Result<Message>;

    async fn followup(&self, command: &Command, message: MessageBuilder) -> Result<()>;

    async fn send(&self, channel_id: ChannelId, message: MessageBuilder) -> Result<Message>;
}

#[async_trait]
impl Bot for Http {
    async fn answer(&self, command: &Command, message: MessageBuilder) -> Result<Message> {
        (command
            .create_interaction_response(self, |response| {
                response
                    .kind(ChannelMessageWithSource)
                    .interaction_response_data(|answer| {
                        answer.allowed_mentions(|mentions| mentions.empty_users());
                        answer.content(message.content);
                        message.files.iter().for_each(|Attachment { file, filename }| {
                            answer.add_file((file.as_slice(), filename.as_str()));
                        });
                        if message.buttons.len() > 0 {
                            answer.components(
                                |components| components.create_action_row(
                                    |action_row|  {
                                        for button in message.buttons {
                                            action_row.create_button(
                                                |b| b
                                                    .custom_id(button.custom_id)
                                                    .style(button.style)
                                                    .label(button.label)
                                            );
                                        }
                                        action_row
                                    }
                                )
                            );    
                        }
                        answer
                    })
            })
            .await)
            .context("Command create response failed")?;
        Ok(command.get_interaction_response(self).await?)
    }

    async fn followup(&self, command: &Command, message: MessageBuilder) -> Result<()> {
        (command
            .create_followup_message(self, |answer| {
                answer.allowed_mentions(|mentions| mentions.empty_users());
                answer.content(message.content);
                message.files.iter().for_each(|Attachment { file, filename }| {
                    answer.add_file((file.as_slice(), filename.as_str()));
                });
                if message.buttons.len() > 0 {
                    answer.components(
                        |components| components.create_action_row(
                            |action_row|  {
                                for button in message.buttons {
                                    action_row.create_button(
                                        |b| b
                                            .custom_id(button.custom_id)
                                            .style(button.style)
                                            .label(button.label)
                                    );
                                }
                                action_row
                            }
                        )
                    );    
                }
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
                    if message.buttons.len() > 0 {
                        answer.components(
                            |components| components.create_action_row(
                                |action_row|  {
                                    for button in message.buttons {
                                        action_row.create_button(
                                            |b| b
                                                .custom_id(button.custom_id)
                                                .style(button.style)
                                                .label(button.label)
                                        );
                                    }
                                    action_row
                                }
                            )
                        );    
                    }
                    answer
                }
            ).await).context("Failed to send message")?
        )
    }
}

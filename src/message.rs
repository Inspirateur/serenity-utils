use serenity::{model::application::component::ButtonStyle, builder::{CreateMessage, CreateInteractionResponseFollowup, CreateInteractionResponseData}};

pub struct Attachment {
    pub file: Vec<u8>,
    pub filename: String,
}

pub struct Button {
    pub custom_id: String,
    pub style: ButtonStyle,
    pub label: String,
}

#[derive(Default)]
pub struct MessageBuilder {
    pub content: String,
    pub files: Vec<Attachment>,
    pub buttons: Vec<Button>,
    pub ephemeral: bool,
}

impl MessageBuilder {
    pub fn new<S: ToString>(content: S) -> Self {
        MessageBuilder { content: content.to_string(), ..Default::default() }
    }

    pub fn content<S: ToString>(mut self, content: S) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn files(mut self, files: Vec<Attachment>) -> Self {
        self.files = files;
        self
    }
    
    pub fn buttons(mut self, buttons: Vec<Button>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }
}

pub trait MessageUtil {
    fn set_buttons(&mut self, buttons: Vec<Button>) -> &mut Self;
}

impl MessageUtil for CreateMessage<'_> {
    fn set_buttons(&mut self, buttons: Vec<Button>) -> &mut Self {
        if buttons.len() > 0 {
            self.components(
                |components| components.create_action_row(
                    |action_row|  {
                        for button in buttons {
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
        self
    }
}

impl MessageUtil for CreateInteractionResponseData<'_> {
    fn set_buttons(&mut self, buttons: Vec<Button>) -> &mut Self {
        if buttons.len() > 0 {
            self.components(
                |components| components.create_action_row(
                    |action_row|  {
                        for button in buttons {
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
        self
    }
}

impl MessageUtil for CreateInteractionResponseFollowup<'_> {
    fn set_buttons(&mut self, buttons: Vec<Button>) -> &mut Self {
        if buttons.len() > 0 {
            self.components(
                |components| components.create_action_row(
                    |action_row|  {
                        for button in buttons {
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
        self
    }
}
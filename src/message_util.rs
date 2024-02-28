use serenity::{all::{CreateActionRow, CreateAttachment, CreateButton}, model::application::ButtonStyle};

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
    pub files: Vec<CreateAttachment>,
    pub buttons: Vec<CreateActionRow>,
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
        self.files = files
            .into_iter()
            .map(|Attachment { file, filename }| CreateAttachment::bytes(file.as_slice(), filename.as_str()))
            .collect();
        self
    }
    
    pub fn buttons(mut self, buttons: Vec<Button>) -> Self {
        self.buttons = vec![CreateActionRow::Buttons(
            buttons
            .into_iter()
            .map(|button| 
                CreateButton::new(button.custom_id)
                    .style(button.style)
                    .label(button.label)
            )
            .collect()
        )];
        self
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }
    
    pub fn action_rows(&self) -> Vec<CreateActionRow> {
        todo!()
    }
}

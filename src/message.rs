use serenity::model::application::component::ButtonStyle;

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
    pub buttons: Vec<Button>
}

impl MessageBuilder {
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
}
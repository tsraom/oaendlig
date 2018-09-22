pub enum MessageType {
    Static,
    Prompt,
}

pub enum MessagePosition {
    Bottom,
}

pub enum Ui {
    Message(MessageType, MessagePosition, String),
}

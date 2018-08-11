pub struct BottomMessage {
    text: String,
}

impl BottomMessage {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text.as_str()
    }
}

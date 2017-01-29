use super::*;

impl Into<Vec<OutgoingMessage>> for Joke {
    fn into(self) -> Vec<OutgoingMessage> {
        vec![
            OutgoingMessage::new(format!("{}, {}", self.text(), self.copyright()))
        ]
    }
}

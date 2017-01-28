use super::*;

impl Into<Vec<OutgoingMessage>> for Joke {
    fn into(self) -> Vec<OutgoingMessage> {
        vec![
            OutgoingMessage::from(self.text()),
            OutgoingMessage::new(self.copyright())
        ]
    }
}

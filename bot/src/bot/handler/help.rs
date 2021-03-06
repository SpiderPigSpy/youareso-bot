use bot::{IncomingMessage, OutgoingMessage, MessageHandler};
use application::services::*;

pub struct HelpCommand {}

pub fn is_help_command(text: &str) -> bool {
    "/start" == text ||
    "/help" == text ||
    "help" == text || 
    "/помощь" == text
}

impl MessageHandler for HelpCommand {
     fn handle(&self, message: &IncomingMessage, _: &Services) -> Vec<OutgoingMessage> {
         if is_help_command(&message.text) {
            vec![
                OutgoingMessage::new(format!("Здесь собираются всякие шутки.
Отправь /youareso чтобы получить случайную шутку.
Отправь название объекта насмешки, или свойства чтобы получить определенную шутку.
Например, \"тупой\", \"толстый\" или \"подружка\", \"парень\".
Чтобы добавить новую шутку, просто отправь мне ее. 
Но сначала рекомендую почитать существующие, чтобы понять что к чему."))
            ]
         } else {
             vec![]
         }
     }
}

impl HelpCommand {
     pub fn new() -> HelpCommand { HelpCommand{} }
}

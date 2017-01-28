use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use application::services::*;
use domain::service::joke::*;

pub struct UnknownCommandHandler<'r> {
    services: &'r Services<'r>
}

impl<'r> MessageHandler for UnknownCommandHandler<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if super::random::is_random_joke_command(&message.text) {
             return vec![];
         }
         if super::help::is_help_command(&message.text) {
             return vec![];
         }
         if self.services.adjective_repository.find_one_by_value(&message.text).is_some() {
             return vec![];
         }
         if self.services.subject_repository.find_one_by_value(&message.text).is_some() {
             return vec![];
         }
         if NewJokeParams::parse(&message.text).is_some() {
             return vec![];
         }
         vec![
             OutgoingMessage::new(format!(
                 "Ты не хочешь случайную шутку;
не похоже, что ты хочешь добавить новую шутку;
я не нашел такого объекта насмешки;
я не нашел токого свойства насмешки.
Чего же ты хочешь? Попробуй /help"
                ))
         ]
     }
}

impl<'r> UnknownCommandHandler<'r> {
     pub fn new(services: &'r Services) -> UnknownCommandHandler<'r> { 
         UnknownCommandHandler{
            services: services
        } 
     }
}

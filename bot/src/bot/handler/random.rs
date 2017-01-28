use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use application::services::*;

pub struct Random<'r> {
    services: &'r Services<'r>
}

pub fn is_random_joke_command(text: &str) -> bool {
    "/youareso" == text
}

impl<'r> MessageHandler for Random<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if is_random_joke_command(&message.text) {
             return self.services.joke_repository.find_one_random().into();
         }
         vec![
             
         ]
     }
}

impl<'r> Random<'r> {
     pub fn new(services: &'r Services) -> Random<'r> { 
         Random{
            services: services
        } 
     }
}

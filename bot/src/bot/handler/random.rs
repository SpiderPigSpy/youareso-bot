use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use application::services::*;

pub struct Random {
    
}

pub fn is_random_joke_command(text: &str) -> bool {
    "/youareso" == text
}

impl MessageHandler for Random {
     fn handle(&self, message: &IncomingMessage, services: &Services) -> Vec<OutgoingMessage> {
         if is_random_joke_command(&message.text) {
             return services.joke_repository.find_one_random().into();
         }
         vec![
             
         ]
     }
}

impl Random {
     pub fn new() -> Random { 
         Random{
        } 
     }
}

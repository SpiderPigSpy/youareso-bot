use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use application::services::*;

pub struct RandomByAdjective<'r> {
    services: &'r Services<'r>
}

impl<'r> MessageHandler for RandomByAdjective<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if let Some(adjective) = self.services.adjective_repository.find_one_by_value(&message.text) {
             if let Some(joke) = self.services.joke_repository.find_one_random_by_adjective(&adjective) {
                 return joke.into();
             }
         }
         vec![]
     }
}

impl<'r> RandomByAdjective<'r> {
     pub fn new(services: &'r Services) -> RandomByAdjective<'r> { 
         RandomByAdjective{
            services: services
        } 
     }
}

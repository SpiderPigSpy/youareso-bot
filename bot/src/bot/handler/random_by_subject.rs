use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use application::services::*;

pub struct RandomBySubject<'r> {
    services: &'r Services<'r>
}

impl<'r> MessageHandler for RandomBySubject<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if let Some(subject) = self.services.subject_repository.find_one_by_value(&message.text) {
             if let Some(joke) = self.services.joke_repository.find_one_random_by_subject(&subject) {
                 return joke.into();
             }
         }
         vec![]
     }
}

impl<'r> RandomBySubject<'r> {
     pub fn new(services: &'r Services) -> RandomBySubject<'r> { 
         RandomBySubject{
            services: services
        } 
     }
}

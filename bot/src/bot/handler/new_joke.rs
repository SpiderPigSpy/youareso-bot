use accord::{Accord, MultipleError};

use bot::{IncomingMessage, OutgoingMessage, MessageHandler};
use application::services::*;
use domain::*;
use domain::service::joke::*;

pub struct NewJokeHandler<'r> {
    services: &'r Services<'r>
}

impl<'r> MessageHandler for NewJokeHandler<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if let Some(new_joke_params) = NewJokeParams::parse(&message.text) {
             if let Err(multiple_error) = new_joke_params.validate() {
                 return Self::validation_error_message(multiple_error);
             }
             if let Some(alreay_existing_same_joke) = self.find_existing(&new_joke_params) {
                 return Self::already_exists_message(alreay_existing_same_joke);
             }
             return into_messages(self.create_new(new_joke_params, message.user.clone()));
         }
         vec![]
     }
}

impl<'r> NewJokeHandler<'r> {
     pub fn new(services: &'r Services) -> NewJokeHandler<'r> { 
         NewJokeHandler{
            services: services
        } 
     }

     fn find_existing(&self, new_joke_params: &NewJokeParams) -> Option<Joke> {
         self.services.joke_repository.find_one_by_text(&new_joke_params.text)
     }

     fn create_new(&self, new_joke_params: NewJokeParams, author: User) -> Option<Joke> {
         let subject = service::subject::find_or_create(
             &new_joke_params.subject, 
             &self.services.subject_repository
         );
         let adjective = service::adjective::find_or_create(
             &new_joke_params.adjective, 
             &self.services.adjective_repository
         );
         let new_joke = self.services.joke_repository.create(
            NewJoke::new(author, subject, adjective, new_joke_params.text)
         );
         Some(new_joke)
     }

     fn already_exists_message(alreay_existing_same_joke: Joke) -> Vec<OutgoingMessage> {
        vec![
            OutgoingMessage::new(format!("Точно такая же шутка уже была добавлена вот им: {}", alreay_existing_same_joke.author().username()))
        ]
     }

     fn validation_error_message(multiple_error: MultipleError) -> Vec<OutgoingMessage> {
         let mut error_message = format!("Извини, не удалось сохранить твою шутку:\n");
         for error in multiple_error.0 {
             error_message.push_str(&error.tag);
             error_message.push_str(": ");
             for invalid in error.invalids {
                 error_message.push_str(&invalid.msg);
                 error_message.push_str(";");
                 //TODO proper
             }
         }
         vec![OutgoingMessage::new(error_message)]
     }
}

fn into_messages(new_joke: Option<Joke>) -> Vec<OutgoingMessage> {
    match new_joke {
        Some(joke) => {
            vec![
                OutgoingMessage::from("Добавил новую шутку. Спасибо."),
                OutgoingMessage::from(joke.text()),
                OutgoingMessage::new(joke.copyright()),
            ]
        },
        None => {
            vec![
                OutgoingMessage::from("Чтото при добавлении шутки пошло не так...")
            ]
        }
    }
}



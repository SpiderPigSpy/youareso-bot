use telegram_bot::*;
use domain::User as DomainUser;

use domain::*;
use application::services::*;

pub mod handler;
mod conversions;

pub struct Bot<'r> {
    api: &'r Api,
    handlers: &'r [&'r MessageHandler],
    services: &'r Services<'r>
}

impl<'r> Bot<'r> {
    pub fn new(api: &'r Api, services: &'r Services, handlers: &'r [&'r MessageHandler]) -> Bot<'r> {
        Bot {
            api: api,
            services: services,
            handlers: handlers
        }
    }

    pub fn process_message(&self, message: Message) {
        let chat_id = message.chat.id();
        if let Some(incoming_message) = IncomingMessage::new(message, 
                                                             &self.services.user_repository) {
            for handler in self.handlers {
                self.api.send(chat_id, handler.handle(&incoming_message));
            }
        }
    }
}

pub struct IncomingMessage {
    user: DomainUser,
    text: String
}

impl IncomingMessage {
     fn new(message: Message, user_repo: &UserRepository) -> Option<IncomingMessage> {
         if let &MessageType::Text(ref text) = &message.msg {
            let username = &message.from.username.unwrap_or(message.from.first_name) ;
            return Some (IncomingMessage {
                    user: service::user::find_or_create(message.from.id, username, user_repo),
                    text: text.clone()
            })
         }
         None
     }
}

pub struct OutgoingMessage {
    text: String
}

impl<'r> From<&'r str> for OutgoingMessage {
    fn from(msg: &str) -> Self {
        OutgoingMessage::new(msg.to_owned())
    }
}

impl OutgoingMessage {
    fn new(text: String) -> OutgoingMessage {
        OutgoingMessage {
            text: text
        }
    }
}

pub trait MessageHandler {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage>;
}

trait OutgoingMessageSender {
     fn send(&self, chat_id: i64, messages: Vec<OutgoingMessage>);
}

impl OutgoingMessageSender for Api {
     fn send(&self, chat_id: i64, messages: Vec<OutgoingMessage>) {
         for message in messages {
             self.send_message(chat_id, message.text, None, None, None, None).unwrap();
         }
     }
}

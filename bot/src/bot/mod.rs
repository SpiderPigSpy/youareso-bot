use telegram_bot::*;
use domain::User as DomainUser;

use domain::*;
use application::services::*;
use application::pool::*;

pub mod handler;
mod conversions;

pub struct Bot<'r> {
    api: &'r Api,
    handlers: &'r [&'r MessageHandler],
    pool: ConnectionPool,
}

impl<'r> Bot<'r> {
    pub fn new(api: &'r Api, pool: ConnectionPool, handlers: &'r [&'r MessageHandler]) -> Bot<'r> {
        Bot {
            api: api,
            pool: pool,
            handlers: handlers
        }
    }

    pub fn process_message(&self, message: Message) {
        let chat_id = message.chat.id();
        trace!("trying");
        let conn = self.pool.get();
        trace!("success");
        {
            let services = Services::new(&conn);
            if let Some(incoming_message) = IncomingMessage::new(message, 
                                                                &services.user_repository) {
                let messages: Vec<OutgoingMessage> = self.handlers
                    .iter()
                    .flat_map(|handler| handler.handle(&incoming_message, &services))
                    .collect();
                if messages.is_empty() {
                    self.api.send(chat_id, vec![OutgoingMessage::new(unknown_action_message())]);
                } else {
                    self.api.send(chat_id, messages);
                }
            }
        }
        drop(conn);
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
     fn handle(&self, message: &IncomingMessage, services: &Services) -> Vec<OutgoingMessage>;
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

fn unknown_action_message() -> String {
    format!("Ты не хочешь случайную шутку;
не похоже, что ты хочешь добавить новую шутку;
я не нашел такого объекта насмешки;
я не нашел токого свойства насмешки.
Чего же ты хочешь? Попробуй /help")
}

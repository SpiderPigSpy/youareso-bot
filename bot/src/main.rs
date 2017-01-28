extern crate telegram_bot;
#[macro_use] 
extern crate log;
extern crate env_logger;
extern crate repo;
extern crate diesel;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate accord;
extern crate dotenv;

mod domain;
mod id;
mod bot;
mod application;

use telegram_bot::*;
use diesel::pg::PgConnection;
use diesel::connection::Connection;
use dotenv::dotenv;
use std::env;

use bot::*;
use bot::handler::*;
use application::services::*;

pub fn main() {
    env_logger::init().unwrap();
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    info!("getMe: {:?}", api.get_me());
    let res = listen(api);
    if let Err(e) = res {
        error!("An error occured: {}", e);
    }
}

fn listen(api: Api) -> Result<()> {
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    let conn = establish_connection();
    let services = Services::new(&conn);
    let help = HelpCommand::new();
    let random_subject = RandomBySubject::new(&services);
    let random_adjective = RandomByAdjective::new(&services);
    let new_joke = NewJokeHandler::new(&services);
    let random = Random::new(&services);
    let unknown = UnknownCommandHandler::new(&services);
    let handlers: [&MessageHandler; 6] = [
        &help,
        &random_subject,
        &random_adjective,
        &new_joke,
        &unknown,
        &random
    ];
    let bot = Bot::new(&api, &services, &handlers);
    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
        if let Some(message) = u.message {
            debug!("Raw message: {:?}", &message);
            bot.process_message(message);
        }
        Ok(ListeningAction::Continue)
    });

    return res;
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

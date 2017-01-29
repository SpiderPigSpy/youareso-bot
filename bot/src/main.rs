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
extern crate r2d2;
extern crate r2d2_diesel;

mod domain;
mod id;
mod bot;
mod application;

use telegram_bot::*;
use dotenv::dotenv;
use std::env;

use bot::*;
use bot::handler::*;
use application::services::*;
use application::pool::*;

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
    let help = HelpCommand::new();
    let random_subject = RandomBySubject::new();
    let random_adjective = RandomByAdjective::new();
    let new_joke = NewJokeHandler::new();
    let random = Random::new();
    let handlers: [&MessageHandler; 5] = [
        &help,
        &random_subject,
        &random_adjective,
        &new_joke,
        &random
    ];
    let bot = Bot::new(&api, conn, &handlers);
    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
        trace!("listen");
        if let Some(message) = u.message {
            debug!("Raw message: {:?}", &message);
            bot.process_message(message);
            trace!("sent!");
        }
        Ok(ListeningAction::Continue)
    });
    error!("OUT");
    return res;
}

fn establish_connection() -> ConnectionPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ConnectionPool::new(&database_url)
}

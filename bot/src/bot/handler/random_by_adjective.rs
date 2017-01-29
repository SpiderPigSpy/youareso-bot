use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use application::services::*;
use id::*;

pub struct RandomByAdjective<'r> {
    services: &'r Services<'r>
}

impl<'r> MessageHandler for RandomByAdjective<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if let Some(adjective) = self.find_adjective(&message.text) {
             if let Some(joke) = self.services.joke_repository.find_one_random_by_adjective(&adjective) {
                if message.text.starts_with("/") {
                    return joke.into();
                } else {
                    let mut messages = vec![OutgoingMessage::new(format!("/random_adjective_{}", joke.adjective().id()))];  
                    messages.append(&mut joke.into());
                    return messages;
                }
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
    
    fn find_adjective(&self, query: &str) -> Option<Adjective> {
        match extract_search_param(query) {
            Some(SearchParam::Id(adjective_id)) => self.services.adjective_repository.find_one(adjective_id),
            Some(SearchParam::Value(adjective_value)) => self.services.adjective_repository.find_one_by_value(&adjective_value),
            None => None
        }
    }
}

fn extract_search_param(query: &str) -> Option<SearchParam> {
    if query.starts_with("/") {
        return extract_adjective_id(query);
    }
    Some(SearchParam::Value(query.to_owned().to_lowercase()))
}

fn extract_adjective_id(query: &str) -> Option<SearchParam> {
    use ::regex::Regex;
    use std::str::FromStr;
    lazy_static! {
        static ref RANDOM_SUBJECT: Regex = Regex::new("/random_adjective_(.+)").unwrap();
    }
    if let Some(captures) = RANDOM_SUBJECT.captures(query) {
        return i64::from_str(captures.get(1).unwrap().as_str())
                .ok()
                .map(Id::new)
                .map(|id| SearchParam::Id(id));
    }
    None
}

enum SearchParam {
    Id(AdjectiveId),
    Value(String)
}

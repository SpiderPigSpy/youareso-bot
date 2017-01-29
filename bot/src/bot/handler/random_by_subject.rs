use bot::{IncomingMessage, OutgoingMessage, MessageHandler};

use domain::*;
use id::*;
use application::services::*;

pub struct RandomBySubject<'r> {
    services: &'r Services<'r>
}

impl<'r> MessageHandler for RandomBySubject<'r> {
     fn handle(&self, message: &IncomingMessage) -> Vec<OutgoingMessage> {
         if let Some(subject) = self.find_subject(&message.text) {
             if let Some(joke) = self.services.joke_repository.find_one_random_by_subject(&subject) {
                if message.text.starts_with("/") {
                    return joke.into();
                } else {
                    let mut messages = vec![OutgoingMessage::new(format!("/random_subject_{}", joke.subject().id()))];  
                    messages.append(&mut joke.into());
                    return messages;
                }
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

     fn find_subject(&self, query: &str) -> Option<Subject> {
         match extract_search_param(query) {
             Some(SearchParam::Id(subject_id)) => self.services.subject_repository.find_one(subject_id),
             Some(SearchParam::Value(subject_value)) => self.services.subject_repository.find_one_by_value(&subject_value),
             None => None
         }
     }
}

fn extract_search_param(query: &str) -> Option<SearchParam> {
    if query.starts_with("/") {
        return extract_subject_id(query);
    }
    Some(SearchParam::Value(query.to_owned().to_lowercase()))
}

fn extract_subject_id(query: &str) -> Option<SearchParam> {
    use ::regex::Regex;
    use std::str::FromStr;
    lazy_static! {
        static ref RANDOM_SUBJECT: Regex = Regex::new("/random_subject_(.+)").unwrap();
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
    Id(SubjectId),
    Value(String)
}

use id::*;
use super::*;

pub type JokeId = Id<i64, Joke>;

#[derive(Debug)]
pub struct Joke {
    id: JokeId,
    subject: Subject,
    adjective: Adjective,
    author: User,
    text: String
}

pub struct NewJoke {
    pub subject: Subject,
    pub adjective: Adjective,
    pub author: User,
    pub text: String
}

impl NewJoke {
    pub fn new(author: User, subject: Subject, adjective: Adjective, text: String) -> NewJoke {
        NewJoke {
            subject: subject,
            adjective: adjective,
            author: author,
            text: text
        }
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    pub fn adjective(&self) -> &Adjective {
        &self.adjective
    }

    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Joke {
    pub fn new(id: JokeId, text: String, author: User, subject: Subject, adjective: Adjective) -> Joke {
        Joke {
            id: id,
            text: text,
            author: author,
            subject: subject,
            adjective: adjective
        }
    }

    pub fn author(&self) -> &User {
        &self.author
    }
    
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn copyright(&self) -> String {
        format!("(c) {}", self.author.username())
    }
}

use diesel::pg::PgConnection;

use repo::dao::User as RawUser;
use repo::dao::Adjective as RawAdjective;
use repo::dao::Subject as RawSubject;
use repo::dao::Joke as RawJoke;
use repo::dao::NewJoke as RawNewJoke;

use domain::*;
use id::*;
use application::pool::*;

pub struct PostgresJokeRepository<'r> {
    conn: &'r PgConnection
}

impl<'r> PostgresJokeRepository<'r> {
    pub fn new(conn: &'r PgConnection) -> PostgresJokeRepository {
        PostgresJokeRepository {
            conn: conn
        }
    }

    fn construct_full_joke(&self, raw_joke: RawJoke) -> Joke {
        let raw_author = ::repo::user::find_one_by_id(raw_joke.author_id, self.conn);
        let raw_subject = ::repo::subject::find_one_by_id(raw_joke.subject_id, self.conn);
        let raw_adjective = ::repo::adjective::find_one_by_id(raw_joke.adjective_id, self.conn);
        Joke::from((
            raw_joke,
            raw_author.unwrap(),
            raw_subject.unwrap(),
            raw_adjective.unwrap()
        ))
    }
}

impl<'r> JokeRepository for PostgresJokeRepository<'r> {
    fn find_one(&self, _: JokeId) -> Option<Joke> {
        unimplemented!();
    }
    
    fn create(&self, new_joke: NewJoke) -> Joke {
        let raw_new_joke = RawNewJoke {
            text: new_joke.text().to_owned(),
            author_id: *new_joke.author().id(),
            subject_id: *new_joke.subject().id(),
            adjective_id: *new_joke.adjective().id()
        };
        Joke::from((
            ::repo::joke::create(&raw_new_joke, self.conn),
            new_joke.author().clone(),
            new_joke.subject().clone(),
            new_joke.adjective().clone()
        ))
    }

    fn find_one_random(&self) -> Joke {
        let random_raw_joke = ::repo::joke::find_one_random(self.conn);
        self.construct_full_joke(random_raw_joke)
    }

    fn find_one_by_text(&self, text: &str) -> Option<Joke> {
        ::repo::joke::find_one_by_text(text, self.conn)
            .map(|raw| self.construct_full_joke(raw))
    }

    fn find_one_random_by_subject(&self, subject: &Subject) -> Option<Joke> {
        ::repo::joke::find_one_random_by_subject_id(*subject.id(), self.conn)
            .map(|raw| self.construct_full_joke(raw))
    }
    
    fn find_one_random_by_adjective(&self, adjective: &Adjective) -> Option<Joke> {
        ::repo::joke::find_one_random_by_adjective_id(*adjective.id(), self.conn)
            .map(|raw| self.construct_full_joke(raw))
    }
}

impl From<(RawJoke, RawUser, RawSubject, RawAdjective)> for Joke {
    fn from(composed_joke: (RawJoke, RawUser, RawSubject, RawAdjective)) -> Joke {
        let (raw_joke, raw_user, raw_subject, raw_adjective) = composed_joke;
        Joke::new(
            Id::new(raw_joke.id),
            raw_joke.text,
            User::from(raw_user),
            Subject::from(raw_subject),
            Adjective::from(raw_adjective)
        )
    }
}

impl From<(RawJoke, User, Subject, Adjective)> for Joke {
    fn from(composed_joke: (RawJoke, User, Subject, Adjective)) -> Joke {
        let (raw_joke, author, subject, adjective) = composed_joke;
        Joke::new(
            Id::new(raw_joke.id),
            raw_joke.text,
            author,
            subject,
            adjective
        )
    }
}

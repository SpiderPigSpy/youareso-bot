use domain::*;

pub trait JokeRepository {
     fn find_one(&self, id: JokeId) -> Option<Joke>;
     fn find_one_random(&self) -> Joke;
     fn create(&self, new_joke: NewJoke) -> Joke;
     fn find_one_by_text(&self, text: &str) -> Option<Joke>;
     fn find_one_random_by_subject(&self, subject: &Subject) -> Option<Joke>;
     fn find_one_random_by_adjective(&self, subject: &Adjective) -> Option<Joke>;
}

pub trait AdjectiveRepository {
     fn find_one(&self, id: AdjectiveId) -> Option<Adjective>;
     fn find_one_by_value(&self, value: &str) -> Option<Adjective>;
     fn create(&self, new_adjective: NewAdjective) -> Adjective;
}

pub trait SubjectRepository {
     fn find_one(&self, id: SubjectId) -> Option<Subject>;
     fn find_one_by_value(&self, value: &str) -> Option<Subject>;
     fn create(&self, new_subject: NewSubject) -> Subject;
}

pub trait UserRepository {
     fn find_one(&self, id: UserId) -> Option<User>;
     fn find_one_by_telegram_id(&self, user_telegram_id: i64) -> Option<User>;
     fn create(&self, new_user: NewUser) -> User;
}

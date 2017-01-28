use id::*;

pub type UserId = Id<i64, User>;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    username: String,
    telegram_id: i64
}

pub struct NewUser {
    pub username: String,
    pub telegram_id: i64
}

impl User {
    pub fn new(id: UserId, username: String, telegram_id: i64) -> User {
        User {
            id: id,
            username: username,
            telegram_id: telegram_id
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

     pub fn id(&self) -> UserId {
         self.id
     }
}

impl NewUser {
     pub fn new(username: String,
                telegram_id: i64) -> NewUser {
         NewUser {
            username: username,
            telegram_id: telegram_id
         }
     }
}

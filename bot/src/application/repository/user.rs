use diesel::pg::PgConnection;

use repo::dao::User as RawUser;
use repo::dao::NewUser as RawNewUser;

use domain::*;
use id::*;
use application::pool::*;

pub struct PostgresUserRepository<'r> {
    conn: &'r PgConnection
}

impl<'r> PostgresUserRepository<'r> {
    pub fn new(conn: &'r PgConnection) -> PostgresUserRepository {
        PostgresUserRepository {
            conn: conn
        }
    }
}

impl<'r> UserRepository for PostgresUserRepository<'r> {
    fn find_one(&self, _: UserId) -> Option<User> {
        unimplemented!();
    }

    fn find_one_by_telegram_id(&self, user_telegram_id: i64) -> Option<User> {
        ::repo::user::find_one_by_telegram_id(user_telegram_id, self.conn)
            .map(User::from)
    }
    
    fn create(&self, new_user: NewUser) -> User {
        let raw_new_user = RawNewUser {
            username: new_user.username,
            telegram_id: new_user.telegram_id
        };
        User::from(::repo::user::create(&raw_new_user, self.conn))
    }
}

impl From<RawUser> for User {
    fn from(raw: RawUser) -> User {
        User::new(Id::new(raw.id), raw.username, raw.telegram_id)
    }
}
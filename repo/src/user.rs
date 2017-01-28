use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::*;

pub fn find_one_by_telegram_id(user_telegram_id: i64, conn: &PgConnection) -> Option<User> {
    use schema::user::dsl::*;
    user.filter(telegram_id.eq(user_telegram_id))
        .first(conn)
        .ok()
}

pub fn find_one_by_id(user_id: i64, conn: &PgConnection) -> Option<User> {
    use schema::user::dsl::*;
    user.find(user_id)
        .first(conn)
        .ok()
}

pub fn create(new_user: &NewUser, conn: &PgConnection) -> User {
    use schema::user;
    ::diesel::insert(new_user)
        .into(user::table)
        .get_result::<User>(conn)
        .expect("Error saving new user")
}

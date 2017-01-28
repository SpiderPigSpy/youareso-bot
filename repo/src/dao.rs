use schema::user;
use schema::joke;
use schema::subject;
use schema::adjective;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub telegram_id: i64
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser {
    pub username: String,
    pub telegram_id: i64
}

#[derive(Queryable, Debug, Clone)]
pub struct Subject {
    pub id: i64,
    pub value: String
}

#[derive(Insertable)]
#[table_name="subject"]
pub struct NewSubject {
    pub value: String
}

#[derive(Queryable, Debug, Clone)]
pub struct Adjective {
    pub id: i64,
    pub value: String
}

#[derive(Insertable)]
#[table_name="adjective"]
pub struct NewAdjective {
    pub value: String
}

#[derive(Queryable, Debug, Clone)]
pub struct Joke {
    pub id: i64,
    pub text: String,
    pub author_id: i64,
    pub subject_id: i64,
    pub adjective_id: i64
}

#[derive(Insertable)]
#[table_name="joke"]
pub struct NewJoke {
    pub text: String,
    pub author_id: i64,
    pub subject_id: i64,
    pub adjective_id: i64
}

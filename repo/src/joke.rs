use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::*;

pub fn find_one_by_id(joke_id: i64, conn: &PgConnection) -> Option<Joke> {
    use schema::joke::dsl::*;
    joke.find(joke_id)
        .first(conn)
        .ok()
}

pub fn find_one_random(conn: &PgConnection) -> Joke {
    use schema::joke::dsl::*;
    use diesel::expression::dsl::count_star;
    let jokes_count: i64 = joke.select(count_star())
        .first(conn)
        .unwrap();
    let random_offset = rand_positive_int() % jokes_count;

    joke.filter(id.is_not_null())
        .offset(random_offset)
        .limit(1)
        .load(conn)
        .unwrap()
        .pop()
        .unwrap()
}

pub fn find_one_random_by_subject_id(joke_subject_id: i64, conn: &PgConnection) -> Option<Joke> {
    use schema::joke::dsl::*;
    joke.filter(subject_id.eq(joke_subject_id))
        .order(::diesel_ext::random_order())
        .limit(1)
        .first(conn)
        .ok()
}

pub fn find_one_random_by_adjective_id(joke_adjective_id: i64, conn: &PgConnection) -> Option<Joke> {
    use schema::joke::dsl::*;
    joke.filter(adjective_id.eq(joke_adjective_id))
        .order(::diesel_ext::random_order())
        .limit(1)
        .first(conn)
        .ok()
}

pub fn create(new_joke: &NewJoke, conn: &PgConnection) -> Joke {
    use schema::joke;
    ::diesel::insert(new_joke)
        .into(joke::table)
        .get_result(conn)
        .expect("Error saving new joke")
}

pub fn find_one_by_text(joke_text: &str, conn: &PgConnection) -> Option<Joke> {
    use schema::joke::dsl::*;
    joke.filter(text.eq(joke_text))
        .first(conn)
        .ok()
}

fn rand_positive_int() -> i64 {
    use rand::Rng;
    let initial = ::rand::thread_rng().gen::<i64>();
    if initial > 0 { initial } else  { -initial }
}

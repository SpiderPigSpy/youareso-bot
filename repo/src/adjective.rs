use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::*;

pub fn find_one_by_id(adjective_id: i64, conn: &PgConnection) -> Option<Adjective> {
    use schema::adjective::dsl::*;
    adjective.find(adjective_id)
        .first(conn)
        .ok()
}

pub fn find_one_by_value(adjective_value: &str, conn: &PgConnection) -> Option<Adjective> {
    use schema::adjective::dsl::*;
    adjective.filter(value.eq(adjective_value))
        .first(conn)
        .ok()
}

pub fn create(new_adjective: &NewAdjective, conn: &PgConnection) -> Adjective {
    use schema::adjective;
    ::diesel::insert(new_adjective)
        .into(adjective::table)
        .get_result(conn)
        .expect("Error saving new adjective")
}

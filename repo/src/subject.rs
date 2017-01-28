use diesel::pg::PgConnection;
use diesel::prelude::*;
use dao::*;

pub fn find_one_by_id(subject_id: i64, conn: &PgConnection) -> Option<Subject> {
    use schema::subject::dsl::*;
    subject.find(subject_id)
        .first(conn)
        .ok()
}

pub fn find_one_by_value(subject_value: &str, conn: &PgConnection) -> Option<Subject> {
    use schema::subject::dsl::*;
    subject.filter(value.eq(subject_value))
        .first(conn)
        .ok()
}

pub fn create(new_subject: &NewSubject, conn: &PgConnection) -> Subject {
    use schema::subject;
    ::diesel::insert(new_subject)
        .into(subject::table)
        .get_result(conn)
        .expect("Error saving new subject")
}

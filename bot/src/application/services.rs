use diesel::pg::PgConnection;

use application::repository::*;

pub struct Services<'r> {
    pub user_repository: PostgresUserRepository<'r>,
    pub subject_repository: PostgresSubjectRepository<'r>,
    pub adjective_repository: PostgresAdjectiveRepository<'r>,
    pub joke_repository: PostgresJokeRepository<'r>
}

impl<'r> Services<'r> {
    pub fn new(conn: &'r PgConnection) -> Services<'r> {
        Services {
            user_repository: PostgresUserRepository::new(conn),
            subject_repository: PostgresSubjectRepository::new(conn),
            adjective_repository: PostgresAdjectiveRepository::new(conn),
            joke_repository: PostgresJokeRepository::new(conn)
        }
    }
}

use diesel::pg::PgConnection;

use repo::dao::Subject as RawSubject;
use repo::dao::NewSubject as RawNewSubject;

use domain::*;
use id::*;

pub struct PostgresSubjectRepository<'r> {
    conn: &'r PgConnection
}

impl<'r> PostgresSubjectRepository<'r> {
    pub fn new(conn: &'r PgConnection) -> PostgresSubjectRepository {
        PostgresSubjectRepository {
            conn: conn
        }
    }
}

impl<'r> SubjectRepository for PostgresSubjectRepository<'r> {
    fn find_one(&self, id: SubjectId) -> Option<Subject> {
        ::repo::subject::find_one_by_id(*id, self.conn)
            .map(Subject::from)
    }

    fn find_one_by_value(&self, value: &str) -> Option<Subject> {
        ::repo::subject::find_one_by_value(value, self.conn)
            .map(Subject::from)
    }
    
    fn create(&self, new_subject: NewSubject) -> Subject {
        let raw_new_subject = RawNewSubject {
            value: new_subject.value
        };
        Subject::from(::repo::subject::create(&raw_new_subject, self.conn))
    }
}

impl From<RawSubject> for Subject {
    fn from(raw: RawSubject) -> Subject {
        Subject::new(Id::new(raw.id), raw.value)
    }
}

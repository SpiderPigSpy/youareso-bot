use diesel::pg::PgConnection;

use repo::dao::Adjective as RawAdjective;
use repo::dao::NewAdjective as RawNewAdjective;

use domain::*;
use id::*;
use application::pool::*;

pub struct PostgresAdjectiveRepository<'r> {
    conn: &'r PgConnection
}

impl<'r> PostgresAdjectiveRepository<'r> {
    pub fn new(conn: &'r PgConnection) -> PostgresAdjectiveRepository {
        PostgresAdjectiveRepository {
            conn: conn
        }
    }
}

impl<'r> AdjectiveRepository for PostgresAdjectiveRepository<'r> {
    fn find_one(&self, id: AdjectiveId) -> Option<Adjective> {
        ::repo::adjective::find_one_by_id(*id, self.conn)
            .map(Adjective::from)
    }

    fn find_one_by_value(&self, value: &str) -> Option<Adjective> {
        ::repo::adjective::find_one_by_value(value, self.conn)
            .map(Adjective::from)
    }
    
    fn create(&self, new_adjective: NewAdjective) -> Adjective {
        let raw_new_adjective = RawNewAdjective {
            value: new_adjective.value
        };
        Adjective::from(::repo::adjective::create(&raw_new_adjective, self.conn))
    }
}

impl From<RawAdjective> for Adjective {
    fn from(raw: RawAdjective) -> Adjective {
        Adjective::new(Id::new(raw.id), raw.value)
    }
}

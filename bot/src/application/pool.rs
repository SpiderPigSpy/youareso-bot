use diesel::pg::PgConnection;
use diesel::connection::Connection;

pub struct ConnectionPool {
    conn: PgConnection
}

impl ConnectionPool {
    pub fn new(connection_string: &str) -> ConnectionPool {
        ConnectionPool {
            conn: PgConnection::establish(connection_string).unwrap()
        }
    }

    pub fn get(&self) -> &PgConnection {
        &self.conn
    }
}

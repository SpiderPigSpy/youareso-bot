use diesel::pg::PgConnection;
use diesel::connection::Connection;

pub struct ConnectionPool {
    connection_string: String,
    conn: PgConnection
}

impl ConnectionPool {
    pub fn new(connection_string: &str) -> ConnectionPool {
        ConnectionPool {
            connection_string: connection_string.to_owned(),
            conn: PgConnection::establish(connection_string).unwrap()
        }
    }

    pub fn get(&self) -> &PgConnection {
        &self.conn
    }
}

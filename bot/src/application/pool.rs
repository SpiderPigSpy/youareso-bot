use diesel::pg::PgConnection;
use diesel::connection::Connection;

pub struct ConnectionPool {
    connection_string: String
}

impl ConnectionPool {
    pub fn new(connection_string: &str) -> ConnectionPool {
        // let config = ::r2d2::Config::builder()
        //     .pool_size(10)
        //     .helper_threads(1)
        //     .build();
        // let manager = ConnectionManager::<PgConnection>::new(connection_string);
        // let pool = ::r2d2::Pool::new(config, manager).expect("Failed to create pool.");
        ConnectionPool {
            connection_string: connection_string.to_owned()
        }
    }

    pub fn get(&self) -> PgConnection {
        PgConnection::establish(&self.connection_string).unwrap()
    }
}

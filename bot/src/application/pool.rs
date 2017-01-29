use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use r2d2::PooledConnection;

pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl ConnectionPool {
    pub fn new(connection_string: &str) -> ConnectionPool {
        let config = ::r2d2::Config::builder()
            .pool_size(1)
            .helper_threads(1)
            .build();
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        let pool = ::r2d2::Pool::new(config, manager).expect("Failed to create pool.");
        ConnectionPool {
            pool: pool
        }
    }

    pub fn get(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
}

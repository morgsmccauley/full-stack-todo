use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub type DbPoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(connspec);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool }
    }
}

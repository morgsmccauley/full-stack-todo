use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub type DbPoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn create_pool() -> DbPool {
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

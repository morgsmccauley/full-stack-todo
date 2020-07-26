use crate::database::pool::DbPoolConnection;

pub struct Context {
    pub db_conn: DbPoolConnection,
}

impl juniper::Context for Context {}

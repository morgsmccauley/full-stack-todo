#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use std::io;

use routes::route_config;

mod database;
mod graphql;
mod models;
mod routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::new().finish())
            .configure(route_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

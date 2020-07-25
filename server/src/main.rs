#[macro_use]
extern crate diesel;

use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use schema::{Context, DbPool};

mod diesel_schema;
mod schema;

use crate::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("http://127.0.0.1:8080/graphql"))
}

async fn graphql(
    schema: web::Data<Arc<Schema>>,
    pool: web::Data<DbPool>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let context = Context {
        db: pool.get().expect("Couldn't get db connection from pool"),
    };

    Ok(HttpResponse::Ok().content_type("application/json").body(
        web::block(move || {
            let res = data.execute(&schema, &context);
            Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
        })
        .await?,
    ))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

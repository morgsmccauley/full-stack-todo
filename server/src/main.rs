#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::io;
use std::sync::Arc;

use database::{Database, DbPool};
use graphql_schema::{Context, Schema, create_schema};

mod database;
mod diesel_schema;
mod graphql_schema;

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
        db_conn: pool.get().expect("Couldn't get db connection from pool"),
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

    let schema = std::sync::Arc::new(create_schema());
    let database = Database::new();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(database.pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

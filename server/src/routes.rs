use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use std::sync::Arc;

use crate::database::actions::Actions;
use crate::database::pool::{create_pool, DbPool};
use crate::graphql::context::Context;
use crate::graphql::schema::{create_schema, Schema};

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
    let db_conn = pool.get().expect("Couldn't get db connection from pool");
    let context = Context {
        actions: Actions::new(db_conn),
    };

    Ok(HttpResponse::Ok().content_type("application/json").body(
        web::block(move || {
            let res = data.execute(&schema, &context);
            Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
        })
        .await?,
    ))
}

pub fn route_config(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(create_schema());
    let pool = create_pool();

    config
        .data(schema.clone())
        .data(pool.clone())
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)));
}

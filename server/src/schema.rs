use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use juniper::{FieldResult, GraphQLInputObject, GraphQLObject, RootNode};
use serde::Serialize;

#[derive(GraphQLObject, Debug, Clone, Serialize, Queryable)]
pub struct ToDo {
    id: String,
    label: String,
}

#[derive(GraphQLInputObject)]
struct NewToDo {
    label: String,
}

pub struct QueryRoot;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub type DbPoolConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct Context {
    pub db: DbPoolConnection,
}

impl juniper::Context for Context {}

#[juniper::object(context = Context)]
impl QueryRoot {
    fn to_do(context: &Context, uuid: String) -> FieldResult<ToDo> {
        use crate::diesel_schema::todo::dsl::*;
        Ok(todo.filter(id.eq(uuid)).first::<ToDo>(&context.db)?)
    }
}

pub struct MutationRoot;

#[juniper::object(context = Context)]
impl MutationRoot {
    fn add_to_do(context: &Context) -> FieldResult<ToDo> {
        Ok(ToDo {
            id: "1".to_owned(),
            label: "Groceries".to_owned(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

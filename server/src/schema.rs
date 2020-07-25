use diesel::prelude::*;
use juniper::{FieldResult, GraphQLObject, RootNode};
use serde::Serialize;
use uuid::Uuid;
use crate::diesel_schema::to_dos::{self, dsl};
use crate::database::DbPoolConnection;

#[derive(GraphQLObject, Debug, Clone, Serialize, Queryable, Insertable)]
pub struct ToDo {
    id: String,
    label: String,
}

pub struct Context {
    pub db_conn: DbPoolConnection,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(context = Context)]
impl QueryRoot {
    fn to_do(context: &Context, id: String) -> FieldResult<ToDo> {
        Ok(dsl::to_dos.filter(dsl::id.eq(id)).first::<ToDo>(&context.db_conn)?)
    }
}

pub struct MutationRoot;

#[juniper::object(context = Context)]
impl MutationRoot {
    fn add_to_do(context: &Context, label: String) -> FieldResult<ToDo> {
        let to_do = ToDo {
            id: Uuid::new_v4().to_string(),
            label,
        };
        diesel::insert_into(dsl::to_dos).values(&to_do).execute(&context.db_conn)?;
        Ok(to_do)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

use crate::database::DbPoolConnection;
use crate::diesel_schema::to_dos::{self, dsl};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLObject, RootNode};
use serde::Serialize;
use uuid::Uuid;

#[derive(GraphQLObject, Debug, Clone, Serialize, Queryable, Insertable, AsChangeset)]
pub struct ToDo {
    id: String,
    label: String,
    done: bool,
}

pub struct Context {
    pub db_conn: DbPoolConnection,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(context = Context)]
impl Query {
    fn to_do(context: &Context, id: String) -> FieldResult<ToDo> {
        Ok(dsl::to_dos.find(id).first::<ToDo>(&context.db_conn)?)
    }

    fn to_dos(context: &Context) -> FieldResult<Vec<ToDo>> {
        Ok(dsl::to_dos.load::<ToDo>(&context.db_conn)?)
    }
}

pub struct Mutation;

#[juniper::object(context = Context)]
impl Mutation {
    fn add_to_do(context: &Context, label: String) -> FieldResult<ToDo> {
        let to_do = ToDo {
            id: Uuid::new_v4().to_string(),
            done: false,
            label,
        };

        diesel::insert_into(dsl::to_dos)
            .values(&to_do)
            .execute(&context.db_conn)?;

        Ok(to_do)
    }

    fn update_to_do(
        context: &Context,
        id: String,
        label: Option<String>,
        done: Option<bool>,
    ) -> FieldResult<ToDo> {
        let to_do = dsl::to_dos
            .find(id.clone())
            .first::<ToDo>(&context.db_conn)?;

        let updated_to_do = ToDo {
            id,
            label: label.unwrap_or(to_do.label),
            done: done.unwrap_or(to_do.done),
        };

        diesel::update(dsl::to_dos)
            .set(&updated_to_do)
            .execute(&context.db_conn)?;

        Ok(updated_to_do)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

use super::context::Context;
use crate::database::actions::*;
use crate::models::ToDo;
use juniper::{FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
struct DeleteToDoResult {
    ok: bool,
    to_do: ToDo,
}

pub struct Query;

#[juniper::object(context = Context)]
impl Query {
    fn to_do(context: &Context, id: String) -> FieldResult<ToDo> {
        Ok(find_to_do(id, &context.db_conn)?)
    }

    fn to_dos(context: &Context) -> FieldResult<Vec<ToDo>> {
        Ok(load_all_to_dos(&context.db_conn)?)
    }
}

pub struct Mutation;

#[juniper::object(context = Context)]
impl Mutation {
    fn add_to_do(context: &Context, label: String) -> FieldResult<ToDo> {
        Ok(create_to_do(label, &context.db_conn)?)
    }

    fn update_to_do(
        context: &Context,
        id: String,
        label: Option<String>,
        done: Option<bool>,
    ) -> FieldResult<ToDo> {
        Ok(update_to_do(id, label, done, &context.db_conn)?)
    }

    fn delete_to_do(context: &Context, id: String) -> FieldResult<DeleteToDoResult> {
        let to_do = delete_to_do(id, &context.db_conn)?;

        Ok(DeleteToDoResult { ok: true, to_do })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

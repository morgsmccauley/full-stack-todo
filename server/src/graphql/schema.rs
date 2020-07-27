use super::context::Context;
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
        Ok(context.actions.find_to_do(id)?)
    }

    fn to_dos(context: &Context) -> FieldResult<Vec<ToDo>> {
        Ok(context.actions.load_all_to_dos()?)
    }
}

pub struct Mutation;

#[juniper::object(context = Context)]
impl Mutation {
    fn add_to_do(context: &Context, label: String) -> FieldResult<ToDo> {
        Ok(context.actions.create_to_do(label)?)
    }

    fn update_to_do(
        context: &Context,
        id: String,
        label: Option<String>,
        done: Option<bool>,
    ) -> FieldResult<ToDo> {
        Ok(context.actions.update_to_do(id, label, done)?)
    }

    fn delete_to_do(context: &Context, id: String) -> FieldResult<DeleteToDoResult> {
        let to_do = context.actions.delete_to_do(id)?;

        Ok(DeleteToDoResult { ok: true, to_do })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

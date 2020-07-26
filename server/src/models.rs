use juniper::GraphQLObject;
use serde::Serialize;
use crate::database::schema::to_dos;

#[derive(GraphQLObject, Debug, Clone, Serialize, Queryable, Insertable, AsChangeset)]
pub struct ToDo {
    pub id: String,
    pub label: String,
    pub done: bool,
}

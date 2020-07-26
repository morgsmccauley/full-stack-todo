use crate::database::schema::to_dos;
use juniper::GraphQLObject;
use serde::Serialize;

#[derive(GraphQLObject, Debug, Clone, Serialize, Queryable, Insertable, AsChangeset)]
pub struct ToDo {
    pub id: String,
    pub label: String,
    pub done: bool,
}

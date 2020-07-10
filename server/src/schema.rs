use juniper::{RootNode, FieldResult, GraphQLObject, GraphQLInputObject};

#[derive(GraphQLObject)]
struct ToDo {
    id: String,
    done: bool,
    label: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewToDo {
    label: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn to_do_list() -> FieldResult<Vec<ToDo>> {
        Ok(vec![
            ToDo {
                id: "1".to_owned(),
                label: "Groceries".to_owned(),
                done: false,
            },
        ])
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn add_to_do(to_do: NewToDo) -> FieldResult<ToDo> {
        Ok(ToDo {
            id: "1".to_owned(),
            label: "Groceries".to_owned(),
            done: false,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

use uuid::Uuid;
use diesel::prelude::*;
use crate::database::schema::to_dos::dsl;
use crate::database::pool::DbPoolConnection;
use crate::graphql::schema::ToDo;

pub fn find_to_do(id: String, conn: &DbPoolConnection) -> Result<ToDo, diesel::result::Error> {
    dsl::to_dos.find(id).first::<ToDo>(conn)
}

pub fn load_all_to_dos(conn: &DbPoolConnection) -> Result<Vec<ToDo>, diesel::result::Error> {
    dsl::to_dos.load::<ToDo>(conn)
}

pub fn create_to_do(label: String, conn: &DbPoolConnection) -> Result<ToDo, diesel::result::Error> {
    let to_do = ToDo {
        id: Uuid::new_v4().to_string(),
        done: false,
        label,
    };

    diesel::insert_into(dsl::to_dos)
        .values(&to_do)
        .execute(conn)?;

    Ok(to_do)
}

pub fn update_to_do(id: String, label: Option<String>, done: Option<bool>, conn: &DbPoolConnection) -> Result<ToDo, diesel::result::Error> {
    let to_do = find_to_do(id.clone(), conn)?;

    let updated_to_do = ToDo {
        id,
        label: label.unwrap_or(to_do.label),
        done: done.unwrap_or(to_do.done),
    };

    diesel::update(dsl::to_dos)
        .set(&updated_to_do)
        .execute(conn)?;

    Ok(updated_to_do)
}

pub fn delete_to_do(id: String, conn: &DbPoolConnection) -> Result<ToDo, diesel::result::Error> {
    let to_do = find_to_do(id.clone(), conn)?;
    diesel::delete(dsl::to_dos.find(id)).execute(conn)?;

    Ok(to_do)
}

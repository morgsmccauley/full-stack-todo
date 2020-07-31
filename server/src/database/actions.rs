use crate::database::pool::DbPoolConnection;
use crate::database::schema::to_dos::dsl;
use crate::models::ToDo;
use diesel::prelude::*;
use uuid::Uuid;

pub struct Actions {
    db_conn: DbPoolConnection,
}

impl Actions {
    pub fn new(conn: DbPoolConnection) -> Self {
        Actions { db_conn: conn }
    }

    pub fn find_to_do(&self, id: String) -> Result<ToDo, diesel::result::Error> {
        dsl::to_dos.find(id).first::<ToDo>(&self.db_conn)
    }

    pub fn load_all_to_dos(&self) -> Result<Vec<ToDo>, diesel::result::Error> {
        dsl::to_dos.load::<ToDo>(&self.db_conn)
    }

    pub fn create_to_do(&self, label: String) -> Result<ToDo, diesel::result::Error> {
        let to_do = ToDo {
            id: Uuid::new_v4().to_string(),
            done: false,
            label,
        };

        diesel::insert_into(dsl::to_dos)
            .values(&to_do)
            .execute(&self.db_conn)?;

        Ok(to_do)
    }

    pub fn update_to_do(
        &self,
        id: String,
        label: Option<String>,
        done: Option<bool>,
    ) -> Result<ToDo, diesel::result::Error> {
        let to_do = self.find_to_do(id.clone())?;

        let updated_to_do = ToDo {
            id: id.clone(),
            label: label.unwrap_or(to_do.label),
            done: done.unwrap_or(to_do.done),
        };

        diesel::update(dsl::to_dos.find(id))
            .set(&updated_to_do)
            .execute(&self.db_conn)?;

        Ok(updated_to_do)
    }

    pub fn delete_to_do(&self, id: String) -> Result<ToDo, diesel::result::Error> {
        let to_do = self.find_to_do(id.clone())?;
        diesel::delete(dsl::to_dos.find(id)).execute(&self.db_conn)?;

        Ok(to_do)
    }
}

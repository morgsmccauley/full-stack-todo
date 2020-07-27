use crate::database::actions::Actions;

pub struct Context {
    pub actions: Actions,
}

impl juniper::Context for Context {}

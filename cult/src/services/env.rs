use std::rc::Rc;

use crate::{connection::Connection, entities::project};


pub struct Env {
    connection: Rc<Connection>,
}

impl Env {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self { connection }
    }

    pub async fn get (&self, project: String) {
        
    }

    pub async fn get_and_save(&self, project: String) {

    }
}

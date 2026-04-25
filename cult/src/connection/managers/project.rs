use std::{ops::Deref, rc::Rc};

use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::entities::project;

pub struct ProjectManager {
    database: Rc<DatabaseConnection>,
}

impl ProjectManager {
    pub fn new (database: Rc<DatabaseConnection>) -> Self {
        ProjectManager { database }
    }
    
    pub async fn find_all (&self) -> Vec<project::Model> {
        let req = project::Entity::find().all(self.database.deref()).await;
        if let Ok(projects) = req {
            return projects;
        }
        req.unwrap();
        vec![]
    }

    pub async fn create (&self, project_name: String) -> Option<i32> {
        let project = project::ActiveModel {
            name: Set(project_name),
            ..Default::default()
        };
    
        let insert = project::Entity::insert(project).exec(self.database.deref()).await;

        if let Ok(res) = insert {
            return Some(res.last_insert_id);
        }
        
        insert.unwrap();

        None
    }

    pub async fn find_one(&self, name: String) -> Option<project::Model> {
        if let Ok(project) = project::Entity::find_by_name(name).one(self.database.deref()).await {
            return project;
        }
        None
    }
}

use std::{ops::Deref, rc::Rc};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel};
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
        vec![]
    }

    pub async fn create (&self, project_name: String) -> Option<i32> {
        let project = project::ActiveModel {
            name: Set(project_name),
            ..Default::default()
        };
    
        let req = project::Entity::insert(project).exec(self.database.deref()).await;

        if let Ok(res) = req {
            return Some(res.last_insert_id);
        }

        None
    } 

    pub async fn find_one(&self, name: String) -> Option<project::Model> {
        let req = project::Entity::find_by_name(name).one(self.database.deref()).await;
        if let Ok(project) = req {
            return project;
        }
        None
    }
    
    pub async fn remove(&self, name: String) -> bool {
        if let Some(res) = self.find_one(name).await {
            return project::Entity::delete(res.into_active_model())
                .exec(self.database.deref()).await.is_ok();            
        }
        false
    }

    pub async fn rename(&self, name: String, new_name: String) -> bool {
        if let Some(res) = self.find_one(name).await {
            let mut model = res.into_active_model();
            model.name = Set(new_name);
            return project::Entity::update(model).exec(self.database.deref()).await.is_ok();
        }
        false
    }
}

use std::{ops::Deref, rc::Rc};

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

use crate::entities::environment;

pub struct EnvironmentManager {
    database: Rc<DatabaseConnection>
}

impl EnvironmentManager {
    pub fn new(database: Rc<DatabaseConnection>) -> Self {
        EnvironmentManager {
            database
        }
    }
    
    async fn find_by_project(&self, project_id: i32) -> Vec<environment::Model> {
        let req = environment::Entity::find()
            .filter(environment::Column::ProjectId.eq(project_id))
            .all(self.database.deref()).await;
        if let Ok(res) = req {
            return res;
        }
        vec![]
    }

    async fn add(&self, key: String, value: String, project_id: i32) -> Option<i32> {
        let env = environment::ActiveModel {
            key: Set(key),
            value: Set(value),
            project_id: Set(project_id),
            ..Default::default()
        };
        
        let req = environment::Entity::insert(env).exec(self.database.deref()).await;
        
        if let Ok(res) = req {
            return Some(res.last_insert_id);
        }

        None
    }

    async fn remove(&self, key: String, project_id: i32) -> bool {
        return environment::Entity::delete(environment::ActiveModel{
            project_id: Set(project_id),
            key: Set(key),
            ..Default::default()
        }).exec(self.database.deref()).await.is_ok();
    }

    async fn put(&self, key: String, value: String, project_id: i32) -> bool {
        let req = environment::Entity::find()
            .filter(environment::Column::ProjectId.eq(project_id))
            .filter(environment::Column::Key.eq(key))
            .one(self.database.deref()).await;

        if let Ok(res) = req && let Some(env) = res {
            let mut model = env.into_active_model();

            model.value = Set(value);
            return model.update(self.database.deref()).await.is_ok();
        }

        false
    }
}


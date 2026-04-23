use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::{connection::Connection, entities::project};

impl Connection {
  pub async fn test(&self) -> Vec<project::Model> {
    let projects = project::Entity::find().all(&self.database).await.unwrap();
    projects
  }

  pub async fn insert(&self, project_name: String) {
    let project = project::ActiveModel {
      name: Set(project_name),
      ..Default::default()
    };

    project::Entity::insert(project).exec(&self.database).await.unwrap();
  }
}